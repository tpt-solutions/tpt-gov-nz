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
    error::MsdError,
    opa,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "dept_id": "msd" }))
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
) -> Result<Json<ResolveResponse>, MsdError> {
    let row = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MsdError::CitizenNotFound)?;

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
) -> Result<Json<Value>, MsdError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MsdError::CitizenNotFound)?;

    if !is_direct_access(&req.requesting_dept_id) {
        let input = consent::ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &citizen.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        match opa::evaluate(&input).await {
            Some(true) => {}
            Some(false) => return Err(MsdError::ScopeNotGranted(req.scopes.join(","))),
            None => consent::verify_access(&input, None)?,
        }
    }

    let has_scope = |s: &str| req.scopes.contains(&s.to_owned());

    let studylink = if has_scope("msd:studylink") {
        Some(db::fetch_studylink(&pool, citizen.id).await?)
    } else {
        None
    };

    let case_history = if has_scope("msd:case-history") {
        Some(db::fetch_case_history(&pool, citizen.id).await?)
    } else {
        None
    };

    let studylink_json = studylink.and_then(|row| {
        row.map(|s| {
            json!({
                "hasStudentLoan": s.has_student_loan,
                "loanBalance": s.loan_balance.map(|d| d.to_f64().unwrap_or(0.0)),
                "repaymentPlan": s.repayment_plan,
                "hasAllowance": s.has_allowance,
                "allowanceType": s.allowance_type,
                "nextPaymentDate": s.next_payment_date.map(|d| d.to_string()),
                "weeklyAmount": s.weekly_amount.map(|d| d.to_f64().unwrap_or(0.0)),
            })
        })
    });

    let case_history_json = case_history.map(|rows| {
        json!(rows.iter().map(|e| json!({
            "eventDate": e.event_date.to_string(),
            "serviceLine": e.service_line,
            "summary": e.summary,
        })).collect::<Vec<_>>())
    });

    Ok(Json(json!({
        "clientNumber": citizen.client_number,
        "studylink": studylink_json,
        "caseHistory": case_history_json,
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
) -> Result<Json<Value>, MsdError> {
    let citizen = db::resolve_by_did(&pool, &req.did)
        .await?
        .ok_or(MsdError::CitizenNotFound)?;

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
