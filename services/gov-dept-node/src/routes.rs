use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;

use crate::audit::{AuditEvent, Decision, log_access};
use crate::consent::{self, ConsentInput};

pub async fn health() -> Json<serde_json::Value> {
    let dept_id = consent::providing_dept_id();
    Json(serde_json::json!({ "status": "ok", "dept_id": dept_id }))
}

/// Departments that may access citizen data without presenting a federation
/// consent grant (the citizen themselves, and authorised case workers).
fn is_direct_access(requesting_dept_id: &Option<String>) -> bool {
    match requesting_dept_id.as_deref() {
        None | Some("citizen") | Some("staff") => true,
        Some(_) => false,
    }
}

async fn audit(pool: &PgPool, event: AuditEvent) {
    log_access(pool, &event).await;
}

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

pub async fn resolve_citizen(
    State(pool): State<PgPool>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    audit(
        &pool,
        AuditEvent {
            dept_id: consent::providing_dept_id(),
            citizen_did: req.did.clone(),
            action: "resolve".into(),
            requesting_dept: None,
            scopes: vec![],
            decision: Decision::Allowed,
            detail: None,
        },
    )
    .await;

    // Each department overrides this with their own citizen lookup.
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
    /// Set on cross-department / federation requests. Absent for citizen (self) and
    /// staff (case-worker) access, which a department authorises directly.
    #[serde(default)]
    pub requesting_dept_id: Option<String>,
    /// Signed consent grants accompanying a cross-department request.
    #[serde(default)]
    pub consent_grants: Vec<gov_identity_core::DataGrantCredential>,
}

pub async fn fetch_data(
    State(pool): State<PgPool>,
    Json(req): Json<DataRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let dept_id = consent::providing_dept_id();

    // Consent gate: cross-department requests must present valid consent grants
    // covering every requested scope (verified via OPA when available, else locally).
    if !is_direct_access(&req.requesting_dept_id) {
        let input = ConsentInput {
            requesting_dept_id: req.requesting_dept_id.as_deref().unwrap_or("unknown"),
            citizen_did: &req.did,
            requested_scopes: &req.scopes,
            consent_grants: &req.consent_grants,
        };

        let allowed = match crate::opa::evaluate(&input).await {
            Some(true) => true,
            Some(false) => false,
            None => consent::verify_access(&input, &dept_id, None).is_ok(),
        };

        audit(
            &pool,
            AuditEvent {
                dept_id: dept_id.clone(),
                citizen_did: req.did.clone(),
                action: "fetch_data".into(),
                requesting_dept: req.requesting_dept_id.clone(),
                scopes: req.scopes.clone(),
                decision: if allowed {
                    Decision::Allowed
                } else {
                    Decision::Denied
                },
                detail: if allowed {
                    None
                } else {
                    Some(consent::denied_scopes(&input, &dept_id).join(","))
                },
            },
        )
        .await;

        if !allowed {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        audit(
            &pool,
            AuditEvent {
                dept_id: dept_id.clone(),
                citizen_did: req.did.clone(),
                action: "fetch_data".into(),
                requesting_dept: req.requesting_dept_id.clone(),
                scopes: req.scopes.clone(),
                decision: Decision::Allowed,
                detail: None,
            },
        )
        .await;
    }

    // Each department overrides this with their own data lookup.
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Deserialize)]
pub struct ActionRequest {
    pub r#type: String,
    #[allow(dead_code)]
    pub parameters: serde_json::Value,
}

pub async fn submit_action(
    State(pool): State<PgPool>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    audit(
        &pool,
        AuditEvent {
            dept_id: consent::providing_dept_id(),
            citizen_did: "unknown".into(),
            action: "submit_action".into(),
            requesting_dept: None,
            scopes: vec![],
            decision: Decision::Allowed,
            detail: Some(req.r#type.clone()),
        },
    )
    .await;

    // Each department overrides this with their own action handler.
    Err(StatusCode::NOT_IMPLEMENTED)
}
