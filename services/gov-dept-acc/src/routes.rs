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
    error::AccError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "acc" }))
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
) -> Result<Json<ResolveResponse>, AccError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(AccError::CitizenNotFound)?;

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
) -> Result<Json<Value>, AccError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(AccError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(AccError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let claims = if has_scope("acc:claims") {
        Some(db::fetch_claims(&pool, citizen.id).await?)
    } else {
        None
    };

    let entitlements = if has_scope("acc:entitlements") {
        Some(db::fetch_entitlement(&pool, citizen.id).await?)
    } else {
        None
    };

    let rehabilitation = if has_scope("acc:rehabilitation") {
        Some(db::fetch_rehabilitation(&pool, citizen.id).await?)
    } else {
        None
    };

    let claims_json = claims.as_ref().map(|rows| {
        json!(rows.iter().map(|c| json!({
            "claimNumber": c.claim_number,
            "claimType": c.claim_type,
            "status": c.status,
            "injuryDate": c.injury_date.to_string(),
            "description": c.description,
            "weeklyCompensation": c.weekly_compensation,
        })).collect::<Vec<_>>())
    });

    let entitlements_json = entitlements.as_ref().map(|e| {
        json!({
            "hasEntitlement": e.has_entitlement,
            "type": e.r#type,
            "weeklyAmount": e.weekly_amount,
            "remainingWeeks": e.remaining_weeks,
        })
    });

    let rehabilitation_json = rehabilitation.as_ref().map(|rows| {
        json!(rows.iter().map(|r| json!({
            "planId": r.plan_id,
            "description": r.description,
            "status": r.status,
            "provider": r.provider,
            "nextReview": r.next_review.map(|d| d.to_string()),
        })).collect::<Vec<_>>())
    });

    Ok(Json(json!({
        "clientNumber": citizen.client_number,
        "claims": claims_json,
        "entitlements": entitlements_json,
        "rehabilitation": rehabilitation_json,
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
) -> Result<Json<Value>, AccError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(AccError::CitizenNotFound)?;

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
