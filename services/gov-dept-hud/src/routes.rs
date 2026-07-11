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
    error::HudError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "hud" }))
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
) -> Result<Json<ResolveResponse>, HudError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(HudError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.client_number,
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
) -> Result<Json<Value>, HudError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(HudError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(HudError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let applications = if has_scope("hud:applications") {
        Some(db::fetch_applications(&pool, citizen.id).await?)
    } else {
        None
    };

    let tenancies = if has_scope("hud:tenancy") {
        Some(db::fetch_tenancies(&pool, citizen.id).await?)
    } else {
        None
    };

    let maintenance_requests = if has_scope("hud:maintenance") {
        Some(db::fetch_maintenance_requests(&pool, citizen.id).await?)
    } else {
        None
    };

    let applications_json = applications.as_ref().map(|rows| {
        json!(rows.iter().map(|a| json!({
            "applicationNumber": a.application_number,
            "applicationType": a.application_type,
            "status": a.status,
            "priorityBand": a.priority_band,
            "bedroomsNeeded": a.bedrooms_needed,
            "submittedDate": a.submitted_date.to_string(),
        })).collect::<Vec<_>>())
    });

    let tenancies_json = tenancies.as_ref().map(|rows| {
        json!(rows.iter().map(|t| json!({
            "tenancyId": t.tenancy_id,
            "propertyAddress": t.property_address,
            "weeklyRent": t.weekly_rent,
            "incomeRelatedRent": t.income_related_rent,
            "startDate": t.start_date.to_string(),
            "status": t.status,
        })).collect::<Vec<_>>())
    });

    let maintenance_requests_json = maintenance_requests.as_ref().map(|rows| {
        json!(rows.iter().map(|m| json!({
            "requestNumber": m.request_number,
            "category": m.category,
            "status": m.status,
            "description": m.description,
            "requestedDate": m.requested_date.to_string(),
        })).collect::<Vec<_>>())
    });

    Ok(Json(json!({
        "clientNumber": citizen.client_number,
        "applications": applications_json,
        "tenancies": tenancies_json,
        "maintenanceRequests": maintenance_requests_json,
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
) -> Result<Json<Value>, HudError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(HudError::CitizenNotFound)?;

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
