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
    error::DiaError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "dia" }))
}

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
) -> Result<Json<ResolveResponse>, DiaError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(DiaError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.passport_number,
        display_name: None,
    }))
}

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
) -> Result<Json<Value>, DiaError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(DiaError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(DiaError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let passport = if has_scope("dia:passport") {
        Some(db::fetch_passport(&pool, citizen.id).await?)
    } else {
        None
    };

    let birth = if has_scope("dia:documents") {
        Some(db::fetch_birth_cert(&pool, citizen.id).await?)
    } else {
        None
    };

    let citizenship = if has_scope("dia:citizenship") {
        Some(db::fetch_citizenship(&pool, citizen.id).await?)
    } else {
        None
    };

    Ok(Json(json!({
        "passportNumber": citizen.passport_number,
        "passport": passport.as_ref().map(|p| json!({
            "passportNumber": p.passport_number,
            "expiryDate": p.expiry_date.to_string(),
            "renewable": p.renewable,
        })),
        "birthCertificate": birth.as_ref().map(|b| json!({
            "certificateNumber": b.certificate_number,
            "dateOfBirth": b.date_of_birth.to_string(),
            "placeOfBirth": b.place_of_birth,
            "parents": b.parents,
        })),
        "citizenship": citizenship.as_ref().map(|c| json!({
            "status": c.status,
            "certificateNumber": c.certificate_number,
            "grantedAt": c.granted_at.as_ref().map(|d| d.to_string()),
        })),
    })))
}

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
) -> Result<Json<Value>, DiaError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(DiaError::CitizenNotFound)?;

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

pub async fn list_passports(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, DiaError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(DiaError::CitizenNotFound)?;
    let passport = db::fetch_passport(&pool, citizen.id).await?;
    Ok(Json(json!({
        "passports": passport.as_ref().map(|p| vec![json!({
            "passportNumber": p.passport_number,
            "expiryDate": p.expiry_date.to_string(),
            "renewable": p.renewable,
        })]).unwrap_or_default()
    })))
}

pub async fn list_documents(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, DiaError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(DiaError::CitizenNotFound)?;
    let birth = db::fetch_birth_cert(&pool, citizen.id).await?;
    let citizenship = db::fetch_citizenship(&pool, citizen.id).await?;
    Ok(Json(json!({
        "birthCertificate": birth.as_ref().map(|b| json!({
            "certificateNumber": b.certificate_number,
            "dateOfBirth": b.date_of_birth.to_string(),
            "placeOfBirth": b.place_of_birth,
        })),
        "citizenship": citizenship.as_ref().map(|c| json!({
            "status": c.status,
            "certificateNumber": c.certificate_number,
        })),
    })))
}
