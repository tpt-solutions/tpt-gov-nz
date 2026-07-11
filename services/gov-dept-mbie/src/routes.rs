use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    actions,
    consent,
    db,
    error::MbieError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "mbie" }))
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
) -> Result<Json<ResolveResponse>, MbieError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MbieError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.person_id,
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
) -> Result<Json<Value>, MbieError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MbieError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(MbieError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let business_registrations = if has_scope("mbie:business") {
        Some(db::fetch_business_registrations(&pool, citizen.id).await?)
    } else {
        None
    };

    let directorships = if has_scope("mbie:directorships") {
        Some(db::fetch_directorships(&pool, citizen.id).await?)
    } else {
        None
    };

    let business_registrations_json = business_registrations.as_ref().map(|rows| {
        json!(rows.iter().map(|b| json!({
            "nzbn": b.nzbn,
            "entityName": b.entity_name,
            "entityType": b.entity_type,
            "status": b.status,
            "registeredDate": b.registered_date.to_string(),
        })).collect::<Vec<_>>())
    });

    let directorships_json = directorships.as_ref().map(|rows| {
        json!(rows.iter().map(|d| json!({
            "nzbn": d.nzbn,
            "entityName": d.entity_name,
            "role": d.role,
            "appointedDate": d.appointed_date.to_string(),
        })).collect::<Vec<_>>())
    });

    Ok(Json(json!({
        "personId": citizen.person_id,
        "businessRegistrations": business_registrations_json,
        "directorships": directorships_json,
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
) -> Result<Json<Value>, MbieError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MbieError::CitizenNotFound)?;

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
