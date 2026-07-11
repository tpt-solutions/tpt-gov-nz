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
    error::MojError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "moj" }))
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
) -> Result<Json<ResolveResponse>, MojError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MojError::CitizenNotFound)?;

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
) -> Result<Json<Value>, MojError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MojError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(MojError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let fines = if has_scope("moj:fines") {
        Some(db::fetch_fines(&pool, citizen.id).await?)
    } else {
        None
    };

    let disputes = if has_scope("moj:disputes") {
        Some(db::fetch_disputes(&pool, citizen.id).await?)
    } else {
        None
    };

    let court_records = if has_scope("moj:court-records") {
        Some(db::fetch_court_records(&pool, citizen.id).await?)
    } else {
        None
    };

    let fines_json = fines.as_ref().map(|rows| {
        json!(rows.iter().map(|f| json!({
            "fineNumber": f.fine_number,
            "fineType": f.fine_type,
            "status": f.status,
            "amount": f.amount,
            "offenseDate": f.offense_date.to_string(),
            "dueDate": f.due_date.to_string(),
            "description": f.description,
        })).collect::<Vec<_>>())
    });

    let disputes_json = disputes.as_ref().map(|rows| {
        json!(rows.iter().map(|d| json!({
            "disputeNumber": d.dispute_number,
            "claimType": d.claim_type,
            "status": d.status,
            "amountClaimed": d.amount_claimed,
            "hearingDate": d.hearing_date.map(|d| d.to_string()),
            "description": d.description,
        })).collect::<Vec<_>>())
    });

    let court_records_json = court_records.as_ref().map(|rows| {
        json!(rows.iter().map(|c| json!({
            "caseNumber": c.case_number,
            "caseType": c.case_type,
            "status": c.status,
            "nextHearingDate": c.next_hearing_date.map(|d| d.to_string()),
            "description": c.description,
        })).collect::<Vec<_>>())
    });

    Ok(Json(json!({
        "clientNumber": citizen.client_number,
        "fines": fines_json,
        "disputes": disputes_json,
        "courtRecords": court_records_json,
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
) -> Result<Json<Value>, MojError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MojError::CitizenNotFound)?;

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
