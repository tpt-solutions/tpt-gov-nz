use axum::{Json, extract::State, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use sqlx::PgPool;

use crate::audit::{AuditEvent, Decision, log_access};
use crate::consent::{self, ConsentInput};

/// Application state shared across all request handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    /// Ed25519 keypair for signing audit entries.
    pub audit_keypair: std::sync::Arc<gov_federation_core::NodeKeypair>,
}

pub async fn health() -> Json<serde_json::Value> {
    let dept_id = consent::providing_dept_id();
    Json(serde_json::json!({ "status": "ok", "dept_id": dept_id }))
}

/// Departments that may access citizen data without presenting a federation
/// consent grant (the citizen themselves, and authorised case workers).
fn is_direct_access(requesting_dept_id: Option<&str>) -> bool {
    match requesting_dept_id {
        None | Some("citizen") | Some("staff") => true,
        Some(_) => false,
    }
}

/// Extract requesting_dept_id from headers (set by gateway).
/// Falls back to request body for backwards compatibility during transition.
fn extract_requesting_dept(
    headers: &axum::http::HeaderMap,
    body_req: &Option<String>,
) -> Option<String> {
    headers
        .get("x-requesting-dept-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| body_req.clone())
}

/// Write audit entry with fail-closed behavior. Returns error if audit write fails.
async fn audit(
    state: &AppState,
    event: AuditEvent,
) -> Result<(), StatusCode> {
    log_access(&state.pool, &event, Some(&state.audit_keypair))
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e, "audit write failed — rejecting request");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

pub async fn resolve_citizen(
    State(state): State<AppState>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    audit(
        &state,
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
    .await?;

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
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<DataRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let dept_id = consent::providing_dept_id();

    // Extract requesting_dept_id from headers (gateway-set) with fallback to body
    let requesting_dept = extract_requesting_dept(&headers, &req.requesting_dept_id);

    // Consent gate: cross-department requests must present valid consent grants
    // covering every requested scope (verified via OPA when available, else locally).
    if !is_direct_access(requesting_dept.as_deref()) {
        let input = ConsentInput {
            requesting_dept_id: requesting_dept.as_deref().unwrap_or("unknown"),
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
            &state,
            AuditEvent {
                dept_id: dept_id.clone(),
                citizen_did: req.did.clone(),
                action: "fetch_data".into(),
                requesting_dept: requesting_dept.clone(),
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
        .await?;

        if !allowed {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        audit(
            &state,
            AuditEvent {
                dept_id: dept_id.clone(),
                citizen_did: req.did.clone(),
                action: "fetch_data".into(),
                requesting_dept: requesting_dept.clone(),
                scopes: req.scopes.clone(),
                decision: Decision::Allowed,
                detail: None,
            },
        )
        .await?;
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
    State(state): State<AppState>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    audit(
        &state,
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
    .await?;

    // Each department overrides this with their own action handler.
    Err(StatusCode::NOT_IMPLEMENTED)
}
