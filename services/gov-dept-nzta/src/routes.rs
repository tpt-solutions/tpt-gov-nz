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
    error::NztaError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "nzta" }))
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
) -> Result<Json<ResolveResponse>, NztaError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(NztaError::CitizenNotFound)?;

    Ok(Json(ResolveResponse {
        did: row.did,
        dept_local_id: row.driver_licence_number,
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
) -> Result<Json<Value>, NztaError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(NztaError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(NztaError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let driver_licence = if has_scope("nzta:driver-licence") {
        Some(db::fetch_driver_licence(&pool, citizen.id).await?)
    } else {
        None
    };

    let vehicles = if has_scope("nzta:vehicles") {
        Some(db::fetch_vehicles(&pool, citizen.id).await?)
    } else {
        None
    };

    let ruc = if has_scope("nzta:ruc") {
        Some(db::fetch_ruc(&pool, citizen.id).await?)
    } else {
        None
    };

    Ok(Json(json!({
        "driverLicenceNumber": citizen.driver_licence_number,
        "driverLicence": driver_licence.as_ref().map(|dl| dl.as_ref().map(|d| json!({
            "licenceNumber": d.licence_number,
            "fullName": d.full_name,
            "licenceClass": d.licence_class,
            "expiryDate": d.expiry_date.to_string(),
            "conditions": d.conditions,
        }))),
        "vehicles": vehicles.as_ref().map(|vs| vs.iter().map(|v| json!({
            "registration": v.registration,
            "make": v.make,
            "model": v.model,
            "year": v.year,
            "fuelType": v.fuel_type,
            "registrationExpiry": v.registration_expiry.to_string(),
        })).collect::<Vec<_>>()),
        "ruc": ruc.as_ref().map(|rs| rs.iter().map(|r| json!({
            "vehicleRego": r.vehicle_rego,
            "licenceType": r.licence_type,
            "expiryDate": r.expiry_date.to_string(),
            "unitsRemaining": r.units_remaining,
        })).collect::<Vec<_>>()),
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
) -> Result<Json<Value>, NztaError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(NztaError::CitizenNotFound)?;

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

pub async fn list_driver_licences(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, NztaError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(NztaError::CitizenNotFound)?;
    let licence = db::fetch_driver_licence(&pool, citizen.id).await?;
    Ok(Json(json!({
        "driverLicences": licence.as_ref().map(|d| vec![json!({
            "licenceNumber": d.licence_number,
            "fullName": d.full_name,
            "licenceClass": d.licence_class,
            "expiryDate": d.expiry_date.to_string(),
            "conditions": d.conditions,
        })]).unwrap_or_default()
    })))
}

pub async fn list_vehicles(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, NztaError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(NztaError::CitizenNotFound)?;
    let vehicles = db::fetch_vehicles(&pool, citizen.id).await?;
    Ok(Json(json!({
        "vehicles": vehicles.iter().map(|v| json!({
            "registration": v.registration,
            "make": v.make,
            "model": v.model,
            "year": v.year,
            "fuelType": v.fuel_type,
            "registrationExpiry": v.registration_expiry.to_string(),
        })).collect::<Vec<_>>()
    })))
}

pub async fn list_ruc(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<Value>, NztaError> {
    let citizen = db::resolve_by_did(&pool, &did).await?.ok_or(NztaError::CitizenNotFound)?;
    let ruc = db::fetch_ruc(&pool, citizen.id).await?;
    Ok(Json(json!({
        "ruc": ruc.iter().map(|r| json!({
            "vehicleRego": r.vehicle_rego,
            "licenceType": r.licence_type,
            "expiryDate": r.expiry_date.to_string(),
            "unitsRemaining": r.units_remaining,
        })).collect::<Vec<_>>()
    })))
}
