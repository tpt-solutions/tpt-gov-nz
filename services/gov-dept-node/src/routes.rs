use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    dept_id: String,
}

pub async fn health() -> Json<serde_json::Value> {
    let dept_id = std::env::var("TPT__GOV__DEPT_ID").unwrap_or_else(|_| "unknown".into());
    Json(serde_json::json!({ "status": "ok", "dept_id": dept_id }))
}

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

pub async fn resolve_citizen(
    State(_pool): State<PgPool>,
    Json(_req): Json<ResolveRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Each department overrides this with their own citizen lookup
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Deserialize)]
pub struct DataRequest {
    pub did: String,
    pub scopes: Vec<String>,
}

pub async fn fetch_data(
    State(_pool): State<PgPool>,
    Json(_req): Json<DataRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Deserialize)]
pub struct ActionRequest {
    pub r#type: String,
    pub parameters: serde_json::Value,
}

pub async fn submit_action(
    State(_pool): State<PgPool>,
    Json(_req): Json<ActionRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
