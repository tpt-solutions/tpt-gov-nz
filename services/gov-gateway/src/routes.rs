use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "gov-gateway" })
}

#[derive(Deserialize)]
pub struct ResolveRequest {
    pub did: String,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    pub did: String,
    pub resolved: bool,
}

pub async fn citizen_resolve(
    Json(req): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, StatusCode> {
    // Phase 1: stub — full implementation routes to identity server
    if req.did.starts_with("did:gov:nz:") {
        Ok(Json(ResolveResponse { did: req.did, resolved: true }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
