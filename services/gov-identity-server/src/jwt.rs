//! JWT authentication for portal ↔ identity server calls.
//!
//! Portals and department services authenticate to the identity server using
//! HS256-signed JWTs. The shared secret is configured via `TPT__GOV__JWT_SECRET`.
//! This is separate from the `x-identity-api-key` used for internal admin calls.

use std::sync::Arc;

use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::warn;

use crate::state::AppState;

/// Header carrying the JWT for portal/dept service authentication.
pub const JWT_AUTH_HEADER: &str = "authorization";

/// JWT claims for identity server access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityClaims {
    /// Subject: the DID of the caller (portal or dept service).
    pub sub: String,
    /// Audience: must match the identity server's expected audience.
    pub aud: String,
    /// Issuer: the portal/dept service identifier.
    pub iss: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// JWT ID for replay protection.
    pub jti: String,
    /// Scopes granted to this token.
    pub scopes: Vec<String>,
    /// Optional: citizen DID on whose behalf the call is made.
    pub citizen_did: Option<String>,
}

/// Resolve the JWT secret from environment, returning None if not configured.
/// When None, JWT authentication is disabled (local dev only).
pub fn jwt_secret_from_env() -> Option<Arc<str>> {
    std::env::var("TPT__GOV__JWT_SECRET")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| Arc::from(s.trim()))
}

/// Resolve the expected JWT audience from environment.
pub fn jwt_audience_from_env() -> String {
    std::env::var("TPT__GOV__JWT_AUDIENCE").unwrap_or_else(|_| "tpt-gov-nz".into())
}

/// Axum middleware requiring a valid JWT in the Authorization header.
/// If JWT secret is not configured, this middleware allows requests through
/// (for local development only).
pub async fn require_jwt(State(state): State<AppState>, req: Request, next: Next) -> Response {
    // If JWT secret is not configured, skip validation (dev mode).
    let Some(secret) = state.jwt_secret.as_ref() else {
        return next.run(req).await;
    };

    let auth_header = req
        .headers()
        .get(JWT_AUTH_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::trim);

    let token = match auth_header {
        Some(t) if !t.is_empty() => t,
        _ => {
            warn!(path = %req.uri().path(), "identity-server request rejected: missing Authorization header");
            return unauthorized_response("missing or invalid Authorization header");
        }
    };

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[jwt_audience_from_env()]);
    validation.validate_exp = true;

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let claims = match decode::<IdentityClaims>(token, &decoding_key, &validation) {
        Ok(data) => data.claims,
        Err(e) => {
            warn!(path = %req.uri().path(), error = %e, "identity-server request rejected: invalid JWT");
            return unauthorized_response("invalid or expired JWT");
        }
    };

    // Store claims in request extensions for downstream handlers.
    let mut req = req;
    req.extensions_mut().insert(claims);
    next.run(req).await
}

/// Create an unauthorized response.
fn unauthorized_response(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, "Bearer")],
        Json(json!({
            "error": "unauthorized",
            "message": message,
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jwt_secret_from_env_returns_none_when_unset() {
        unsafe {
            std::env::remove_var("TPT__GOV__JWT_SECRET");
        }
        assert!(jwt_secret_from_env().is_none());
    }

    #[test]
    fn jwt_secret_from_env_returns_value_when_set() {
        unsafe {
            std::env::set_var("TPT__GOV__JWT_SECRET", "test-secret-at-least-32-chars-long");
        }
        let secret = jwt_secret_from_env();
        assert!(secret.is_some());
        assert_eq!(
            secret.unwrap().as_ref(),
            "test-secret-at-least-32-chars-long"
        );
        unsafe {
            std::env::remove_var("TPT__GOV__JWT_SECRET");
        }
    }
}
