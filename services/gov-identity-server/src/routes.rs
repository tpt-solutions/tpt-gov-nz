use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "gov-identity-server" })
}

#[derive(Deserialize)]
pub struct RegisterDidRequest {
    pub public_key_b64: String,
}

#[derive(Serialize)]
pub struct RegisterDidResponse {
    pub did: String,
}

pub async fn register_did(
    State(_pool): State<PgPool>,
    Json(_req): Json<RegisterDidRequest>,
) -> Result<Json<RegisterDidResponse>, StatusCode> {
    // Phase 1: stub — full implementation stores DID document in identity-db
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_did_document(
    State(_pool): State<PgPool>,
    axum::extract::Path(_did): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Deserialize)]
pub struct IssueGrantRequest {
    pub citizen_did: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub expires_in_seconds: u64,
}

pub async fn issue_grant(
    State(_pool): State<PgPool>,
    Json(_req): Json<IssueGrantRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
