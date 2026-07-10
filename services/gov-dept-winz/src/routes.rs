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
    error::WinzError,
    opa,
};

// ── Health ────────────────────────────────────────────────────────────────────

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "winz" }))
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
) -> Result<Json<ResolveResponse>, WinzError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(WinzError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.client_id,
        display_name: None,
    }))
}

// ── Fetch consented data ──────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
    /// Set on cross-department / federation requests. Absent for citizen (self) and
    /// staff (case-worker) access, which WINZ authorises directly.
    #[serde(default)]
    pub requesting_dept_id: Option<String>,
    /// Signed consent grants accompanying a cross-department request. Verified
    /// against `policies/winz.rego` (or the local [`crate::consent`] mirror).
    #[serde(default)]
    pub consent_grants: Vec<gov_identity_core::DataGrantCredential>,
}

/// Departments that may access citizen data without presenting a federation
/// consent grant (the citizen themselves, and authorised case workers).
fn is_direct_access(requesting_dept_id: &Option<String>) -> bool {
    match requesting_dept_id.as_deref() {
        None | Some("citizen") | Some("staff") => true,
        Some(_) => false,
    }
}

pub async fn fetch_data(
    State(pool): State<PgPool>,
    Json(req): Json<DataRequest>,
) -> Result<Json<Value>, WinzError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(WinzError::CitizenNotFound)?;

    // Consent gate: cross-department requests must present valid consent grants
    // covering every requested scope (verified via OPA when available, else locally).
    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(WinzError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let benefits = if has_scope("winz:benefits") {
        Some(db::fetch_benefits(&pool, citizen.id).await?)
    } else {
        None
    };

    let payments = if has_scope("winz:payments") {
        Some(db::fetch_payments(&pool, citizen.id).await?)
    } else {
        None
    };

    let case_notes = if has_scope("winz:case-notes") {
        Some(db::fetch_case_notes(&pool, citizen.id).await?)
    } else {
        None
    };

    let active_total = benefits.as_ref().map(|bs| {
        bs.iter()
            .filter(|b| b.status == "active")
            .map(|b| b.weekly_amount)
            .fold(sqlx::types::Decimal::ZERO, |acc, v| acc + v)
    });

    Ok(Json(json!({
        "clientId": citizen.client_id,
        "activeBenefits": benefits.as_ref().map(|bs| bs.iter().map(|b| json!({
            "type": b.benefit_type,
            "weeklyAmount": b.weekly_amount.to_string(),
            "startDate": b.start_date.as_ref().map(|d| d.to_string()),
            "reviewDate": b.review_date.as_ref().map(|d| d.to_string()),
            "status": b.status,
        })).collect::<Vec<_>>()),
        "totalWeeklyPayment": active_total.as_ref().map(|t| t.to_string()),
        "payments": payments.as_ref().map(|ps| ps.iter().map(|p| json!({
            "paymentId": p.id.to_string(),
            "benefitType": p.benefit_type,
            "paymentDate": p.payment_date.to_string(),
            "amount": p.amount.to_string(),
            "method": p.method,
        })).collect::<Vec<_>>()),
        "caseNotes": case_notes.as_ref().map(|ns| ns.iter().map(|n| json!({
            "noteId": n.id.to_string(),
            "noteDate": n.note_date.to_rfc3339(),
            "author": n.author,
            "note": n.note,
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
) -> Result<Json<Value>, WinzError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(WinzError::CitizenNotFound)?;

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

// ── List benefits ─────────────────────────────────────────────────────────────

pub async fn list_benefits(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, WinzError> {
    let citizen = db::resolve_by_did(&pool, &did)
        .await?
        .ok_or(WinzError::CitizenNotFound)?;

    let benefits = db::fetch_benefits(&pool, citizen.id).await?;

    Ok(Json(json!({
        "benefits": benefits.iter().map(|b| json!({
            "type": b.benefit_type,
            "weeklyAmount": b.weekly_amount.to_string(),
            "startDate": b.start_date.as_ref().map(|d| d.to_string()),
            "reviewDate": b.review_date.as_ref().map(|d| d.to_string()),
            "status": b.status,
        })).collect::<Vec<_>>()
    })))
}

// ── List payments ─────────────────────────────────────────────────────────────

pub async fn list_payments(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, WinzError> {
    let citizen = db::resolve_by_did(&pool, &did)
        .await?
        .ok_or(WinzError::CitizenNotFound)?;

    let payments = db::fetch_payments(&pool, citizen.id).await?;

    Ok(Json(json!({
        "payments": payments.iter().map(|p| json!({
            "paymentId": p.id.to_string(),
            "benefitType": p.benefit_type,
            "paymentDate": p.payment_date.to_string(),
            "amount": p.amount.to_string(),
            "method": p.method,
        })).collect::<Vec<_>>()
    })))
}
