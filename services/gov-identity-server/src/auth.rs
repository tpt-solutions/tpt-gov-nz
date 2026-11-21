//! API-key authentication for the identity server's mutation endpoints.
//!
//! The identity server is an internal service: only the portals (server-side)
//! and department services may register DIDs, issue/revoke grants, or read a
//! citizen's grant list. Callers authenticate with the shared secret in
//! `TPT__GOV__IDENTITY_API_KEY`, presented as `x-identity-api-key`.
//!
//! The server refuses to start when the key is unset — there is deliberately
//! no insecure fallback. Local development sets the key in `.env` (see
//! `.env.example`); `docker/phase1.yml` and `docker/demo.yml` wire it for the
//! containers.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::state::AppState;

/// Header carrying the shared identity-server API key.
pub const API_KEY_HEADER: &str = "x-identity-api-key";

/// Constant-time equality for two byte strings.
///
/// Avoids leaking the position of the first differing byte through timing.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Resolve the configured API key, failing startup when it is missing.
pub fn key_from_env() -> anyhow::Result<Arc<str>> {
    match std::env::var("TPT__GOV__IDENTITY_API_KEY") {
        Ok(key) if key.trim().len() >= 16 => Ok(Arc::from(key.trim())),
        Ok(_) => anyhow::bail!(
            "TPT__GOV__IDENTITY_API_KEY must be at least 16 characters (set it in .env)"
        ),
        Err(_) => anyhow::bail!(
            "TPT__GOV__IDENTITY_API_KEY is required — the identity server will not start \
             without authentication (set it in .env; see .env.example)"
        ),
    }
}

/// Axum middleware requiring a valid `x-identity-api-key` header.
pub async fn require_api_key(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let presented = req
        .headers()
        .get(API_KEY_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim);

    match presented {
        Some(key) if constant_time_eq(key.as_bytes(), state.api_key.as_bytes()) => {
            next.run(req).await
        }
        _ => {
            tracing::warn!(
                path = %req.uri().path(),
                "identity-server request rejected: missing or invalid API key"
            );
            (
                StatusCode::UNAUTHORIZED,
                [(header::WWW_AUTHENTICATE, "ApiKey")],
                Json(json!({
                    "error": "unauthorized",
                    "message": "missing or invalid x-identity-api-key",
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::constant_time_eq;

    #[test]
    fn constant_time_eq_matches_normal_equality() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
        assert!(!constant_time_eq(b"", b"a"));
        assert!(constant_time_eq(b"", b""));
    }
}
