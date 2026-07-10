use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    actions,
    consent,
    db,
    error::MohError,
    opa,
};

// ── Health ────────────────────────────────────────────────────────────────────

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "moh" }))
}

// ── Resolve citizen ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    pub did: String,
    #[serde(rename = "deptLocalId")]
    pub dept_local_id: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

pub async fn resolve_citizen(
    State(pool): State<PgPool>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, MohError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MohError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.nhi,
        display_name: None,
    }))
}

// ── Fetch consented data ──────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
    #[serde(default)]
    pub requesting_dept_id: Option<String>,
    #[serde(default)]
    pub consent_grants: Vec<gov_identity_core::DataGrantCredential>,
}

fn is_direct_access(requesting_dept_id: &Option<String>) -> bool {
    match requesting_dept_id.as_deref() {
        None | Some("citizen") | Some("staff") => true,
        Some(_) => false,
    }
}

pub async fn fetch_data(
    State(pool): State<PgPool>,
    Json(req): Json<DataRequest>,
) -> Result<Json<Value>, MohError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MohError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(MohError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let gp = if has_scope("moh:gp") {
        Some(db::fetch_gp_enrolment(&pool, citizen.id).await?)
    } else {
        None
    };

    let prescriptions = if has_scope("moh:prescriptions") {
        Some(db::fetch_prescriptions(&pool, citizen.id).await?)
    } else {
        None
    };

    let appointments = if has_scope("moh:appointments") {
        Some(db::fetch_appointments(&pool, citizen.id).await?)
    } else {
        None
    };

    let vaccinations = if has_scope("moh:vaccinations") {
        Some(db::fetch_vaccinations(&pool, citizen.id).await?)
    } else {
        None
    };

    Ok(Json(json!({
        "nhiNumber": citizen.nhi,
        "enrolledGP": gp.as_ref().map(|g| json!({
            "practiceName": g.practice_name,
            "address": g.address,
            "phone": g.phone,
        })),
        "activePrescriptions": prescriptions.as_ref().map(|ps| ps.iter().map(|p| json!({
            "prescriptionId": p.id.to_string(),
            "medication": p.medication,
            "dose": p.dose,
            "repeatsTotal": p.repeats_total,
            "repeatsRemaining": p.repeats_remaining,
            "issuedAt": p.issued_at.to_string(),
        })).collect::<Vec<_>>()),
        "upcomingAppointments": appointments.as_ref().map(|as_| as_.iter().map(|a| json!({
            "appointmentId": a.id.to_string(),
            "provider": a.provider,
            "date": a.appt_date.to_rfc3339(),
            "type": a.r#type,
            "status": a.status,
        })).collect::<Vec<_>>()),
        "vaccinations": vaccinations.as_ref().map(|vs| vs.iter().map(|v| json!({
            "vaccinationId": v.id.to_string(),
            "vaccine": v.vaccine,
            "date": v.vaccine_date.to_string(),
            "dueForBooster": v.due_for_booster,
        })).collect::<Vec<_>>()),
    })))
}

// ── Submit action ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ActionRequest {
    pub did: String,
    pub r#type: String,
    pub parameters: Value,
    #[serde(default = "default_performed_by")]
    pub performed_by: String,
    pub ai_level: Option<String>,
}

fn default_performed_by() -> String {
    "citizen".to_owned()
}

pub async fn submit_action(
    State(pool): State<PgPool>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<Value>, MohError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MohError::CitizenNotFound)?;

    actions::execute(
        &pool,
        citizen.id,
        &req.r#type,
        &req.parameters,
        &req.performed_by,
        req.ai_level.as_deref(),
    )
    .await
}

// ── List endpoints ─────────────────────────────────────────────────────────────

pub async fn list_prescriptions(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, MohError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(MohError::CitizenNotFound)?;
    let prescriptions = db::fetch_prescriptions(&pool, citizen.id).await?;
    Ok(Json(json!({
        "prescriptions": prescriptions.iter().map(|p| json!({
            "prescriptionId": p.id.to_string(),
            "medication": p.medication,
            "dose": p.dose,
            "repeatsTotal": p.repeats_total,
            "repeatsRemaining": p.repeats_remaining,
            "issuedAt": p.issued_at.to_string(),
        })).collect::<Vec<_>>()
    })))
}

pub async fn list_appointments(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, MohError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(MohError::CitizenNotFound)?;
    let appointments = db::fetch_appointments(&pool, citizen.id).await?;
    Ok(Json(json!({
        "appointments": appointments.iter().map(|a| json!({
            "appointmentId": a.id.to_string(),
            "provider": a.provider,
            "date": a.appt_date.to_rfc3339(),
            "type": a.r#type,
            "status": a.status,
        })).collect::<Vec<_>>()
    })))
}

pub async fn list_vaccinations(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, MohError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(MohError::CitizenNotFound)?;
    let vaccinations = db::fetch_vaccinations(&pool, citizen.id).await?;
    Ok(Json(json!({
        "vaccinations": vaccinations.iter().map(|v| json!({
            "vaccinationId": v.id.to_string(),
            "vaccine": v.vaccine,
            "date": v.vaccine_date.to_string(),
            "dueForBooster": v.due_for_booster,
        })).collect::<Vec<_>>()
    })))
}
