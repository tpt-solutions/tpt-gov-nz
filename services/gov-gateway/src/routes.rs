use axum::{
    Json,
    body::Body,
    extract::{Extension, Path, State},
    http::{HeaderMap, Method, Request, StatusCode, Uri},
};
use http_body_util::{BodyExt, Full, Limited};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::Claims;
use crate::state::AppState;

/// Headers copied from the caller to the upstream department service.
///
/// `authorization` is forwarded so the department service can independently
/// authenticate the citizen; hop-by-hop headers and caller-supplied identity
/// headers are deliberately not forwarded.
const FORWARDED_HEADERS: [&str; 5] = [
    "authorization",
    "content-type",
    "accept",
    "x-request-id",
    "traceparent",
];

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

/// Extract the `dept_id` segment from a `/v1/dept/{dept_id}/...` path.
///
/// Returns a lowercased department id, or `None` if the path is not a dept
/// proxy path. Used by the circuit-breaker middleware, which runs before the
/// typed `Path` extractor in the handler.
pub fn extract_dept_id(path: &str) -> Option<String> {
    let rest = path.strip_prefix("/v1/dept/")?;
    let dept = rest.split('/').next()?;
    if dept.is_empty() {
        None
    } else {
        Some(dept.to_lowercase())
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub departments: Vec<String>,
}

pub async fn health(State(registry): State<DeptRegistry>) -> Json<HealthResponse> {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
struct IdentityDidDocument {
    did: String,
    public_key_b64: String,
    created_at: String,
}

/// Resolve a citizen DID against the identity server.
///
/// The caller may only resolve their own DID: the JWT `sub` must equal the DID
/// in the request body.
pub async fn citizen_resolve(
    State(app): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, StatusCode> {
    if claims.sub != req.did {
        tracing::warn!(
            sub = %claims.sub,
            requested = %req.did,
            "rejected citizen resolve for a different DID"
        );
        return Err(StatusCode::FORBIDDEN);
    }

    let url = format!(
        "{}/v1/did/{}",
        app.identity_url.trim_end_matches('/'),
        req.did
    );
    let uri: Uri = url.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Full::new(axum::body::Bytes::new()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = tokio::time::timeout(app.upstream_timeout, app.http.request(request))
        .await
        .map_err(|_| {
            tracing::warn!(did = %req.did, "identity server resolve timed out");
            StatusCode::GATEWAY_TIMEOUT
        })?
        .map_err(|e| {
            tracing::error!(error = %e, did = %req.did, "identity server resolve failed");
            StatusCode::BAD_GATEWAY
        })?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(Json(ResolveResponse {
            did: req.did,
            resolved: false,
            public_key_b64: None,
            created_at: None,
        }));
    }
    if !response.status().is_success() {
        tracing::error!(
            status = %response.status(),
            did = %req.did,
            "identity server resolve returned an error"
        );
        return Err(StatusCode::BAD_GATEWAY);
    }

    let (_parts, incoming) = response.into_parts();
    let limited = Limited::new(incoming, app.max_body_bytes);
    let bytes = limited
        .collect()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .to_bytes();
    let doc: IdentityDidDocument =
        serde_json::from_slice(&bytes).map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(ResolveResponse {
        did: doc.did,
        resolved: true,
        public_key_b64: Some(doc.public_key_b64),
        created_at: Some(doc.created_at),
    }))
}

// --- Dept proxy ---

fn build_target(base_url: &str, path: &str, query: Option<&str>, https: bool) -> String {
    let base = base_url.trim_end_matches('/');
    let base = if https {
        base.replacen("http://", "https://", 1)
    } else {
        base.to_string()
    };
    let mut target = if path.is_empty() {
        base
    } else {
        format!("{base}/{path}")
    };
    if let Some(q) = query {
        target.push('?');
        target.push_str(q);
    }
    target
}

fn forward_headers(
    builder: axum::http::request::Builder,
    incoming: &HeaderMap,
) -> axum::http::request::Builder {
    let mut builder = builder;
    for name in FORWARDED_HEADERS {
        if let Some(value) = incoming.get(name) {
            builder = builder.header(name, value);
        }
    }
    builder
}

async fn collect_limited(
    incoming: hyper::body::Incoming,
    max_bytes: usize,
) -> Result<axum::body::Bytes, StatusCode> {
    let limited = Limited::new(incoming, max_bytes);
    limited
        .collect()
        .await
        .map(|c| c.to_bytes())
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

pub async fn proxy_dept(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((dept_id, path)): Path<(String, String)>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::response::Response, StatusCode> {
    let dept_id = dept_id.to_lowercase();

    // Caller identity and authorisation come from the JWT, never from
    // caller-supplied headers or body fields.
    if !claims.permits_dept(&dept_id) {
        tracing::warn!(sub = %claims.sub, dept = %dept_id, "caller lacks a scope for department");
        return Err(StatusCode::FORBIDDEN);
    }

    let base_url = app_state
        .registry
        .get_url(&dept_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    let target = build_target(base_url, &path, uri.query(), app_state.mtls.is_some());
    let target_uri: Uri = target.parse().map_err(|_| StatusCode::BAD_GATEWAY)?;

    let mut builder = Request::builder()
        .method(method.clone())
        .uri(target_uri)
        .header("x-dept-id", &dept_id)
        .header("x-citizen-did", &claims.sub);
    builder = forward_headers(builder, &headers);

    // Mutual TLS: when configured, present our client certificate and verify the
    // department service against the internal CA.
    if let Some(mtls) = app_state.mtls.as_ref() {
        let req = builder
            .body(Full::new(body))
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        let resp = tokio::time::timeout(app_state.upstream_timeout, mtls.request(req))
            .await
            .map_err(|_| StatusCode::GATEWAY_TIMEOUT)?
            .map_err(|e| {
                tracing::error!(error = %e, dept = %dept_id, "mTLS proxy request failed");
                StatusCode::BAD_GATEWAY
            })?;
        let (parts, incoming) = resp.into_parts();
        let body_bytes = collect_limited(incoming, app_state.max_body_bytes).await?;
        return Ok(axum::response::Response::from_parts(
            parts,
            axum::body::Body::from(body_bytes),
        ));
    }

    let req = builder
        .body(Full::new(body))
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let resp = tokio::time::timeout(app_state.upstream_timeout, app_state.http.request(req))
        .await
        .map_err(|_| {
            tracing::warn!(dept = %dept_id, "upstream department request timed out");
            StatusCode::GATEWAY_TIMEOUT
        })?
        .map_err(|e| {
            tracing::error!(error = %e, dept = %dept_id, "proxy request failed");
            StatusCode::BAD_GATEWAY
        })?;

    let (parts, incoming) = resp.into_parts();
    let body_bytes = collect_limited(incoming, app_state.max_body_bytes).await?;
    Ok(axum::response::Response::from_parts(
        parts,
        Body::from(body_bytes),
    ))
}

#[cfg(test)]
mod tests {
    use super::{build_target, extract_dept_id};

    #[test]
    fn extracts_dept_id_from_proxy_path() {
        assert_eq!(
            extract_dept_id("/v1/dept/ird/citizen/data").as_deref(),
            Some("ird")
        );
        assert_eq!(
            extract_dept_id("/v1/dept/WINZ/health").as_deref(),
            Some("winz")
        );
        assert_eq!(extract_dept_id("/v1/dept/moh").as_deref(), Some("moh"));
    }

    #[test]
    fn returns_none_for_non_dept_paths() {
        assert_eq!(extract_dept_id("/health"), None);
        assert_eq!(extract_dept_id("/v1/citizen/resolve"), None);
        assert_eq!(extract_dept_id("/v1/dept/"), None);
    }

    #[test]
    fn builds_upstream_target() {
        assert_eq!(
            build_target("http://localhost:8090/", "citizen/data", None, false),
            "http://localhost:8090/citizen/data"
        );
        assert_eq!(
            build_target("http://localhost:8090", "citizen/data", Some("a=1"), false),
            "http://localhost:8090/citizen/data?a=1"
        );
        assert_eq!(
            build_target("http://localhost:8090", "", None, false),
            "http://localhost:8090"
        );
        assert_eq!(
            build_target("http://localhost:8090", "health", None, true),
            "https://localhost:8090/health"
        );
    }
}
