use axum::{
    body::Body,
    extract::{Path, State},
    http::{Method, StatusCode, Uri},
    http::Request,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry mapping department IDs to their backend service URLs.
#[derive(Clone)]
pub struct DeptRegistry {
    departments: HashMap<String, String>,
}

impl DeptRegistry {
    /// Build from env vars: TPT__GOV__DEPT_IRD_URL, TPT__GOV__DEPT_WINZ_URL, etc.
    pub fn from_env() -> Self {
        let mut departments = HashMap::new();
        for (key, value) in std::env::vars() {
            if let Some(dept_id) = key
                .strip_prefix("TPT__GOV__DEPT_")
                .and_then(|s| s.strip_suffix("_URL"))
            {
                departments.insert(dept_id.to_lowercase(), value);
            }
        }
        Self { departments }
    }

    pub fn get_url(&self, dept_id: &str) -> Option<&str> {
        self.departments.get(dept_id).map(|s| s.as_str())
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub departments: Vec<String>,
}

pub async fn health(
    State(registry): State<DeptRegistry>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "gov-gateway",
        departments: registry.departments.keys().cloned().collect(),
    })
}

// --- Citizen resolve (delegates to identity server) ---

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
    // Phase 1: local stub. Full implementation queries gov-identity-server.
    if req.did.starts_with("did:gov:nz:") {
        Ok(Json(ResolveResponse { did: req.did, resolved: true }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// --- Dept proxy ---

pub async fn proxy_dept(
    State(registry): State<DeptRegistry>,
    Path((dept_id, path)): Path<(String, String)>,
    method: Method,
    uri: Uri,
    body: axum::body::Bytes,
) -> Result<axum::response::Response, StatusCode> {
    let base_url = registry
        .get_url(&dept_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    let target = format!("{base_url}/{path}");
    let target_uri: Uri = target.parse().map_err(|_| StatusCode::BAD_GATEWAY)?;

    let mut req_builder = Request::builder()
        .method(method)
        .uri(target_uri)
        .header("x-forwarded-for", "gateway")
        .header("x-dept-id", &dept_id);

    // Forward original query string if present
    if let Some(query) = uri.query() {
        req_builder = req_builder.uri(format!("{target}?{query}"));
    }

    let req = req_builder
        .body(Body::from(body))
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let client = hyper_util::client::legacy::Client::builder(
        hyper_util::rt::TokioExecutor::new(),
    )
    .build_http();

    let resp = client
        .request(req)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, dept = %dept_id, "proxy request failed");
            StatusCode::BAD_GATEWAY
        })?;

    // Convert from hyper Incoming to axum Body
    let (parts, incoming) = resp.into_parts();
    let body_bytes = axum::body::Bytes::from(
        http_body_util::BodyExt::collect(incoming)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?
            .to_bytes(),
    );
    Ok(axum::response::Response::from_parts(parts, axum::body::Body::from(body_bytes)))
}
