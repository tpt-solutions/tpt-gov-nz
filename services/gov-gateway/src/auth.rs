//! JWT validation middleware for the API gateway.
//!
//! Validates a `Bearer` token on protected routes using HS256 and the shared
//! secret in `TPT__GOV__JWT_SECRET`. On success the decoded [`Claims`] are
//! inserted into request extensions so downstream handlers can read the
//! citizen DID and granted scopes.
//!
//! For Phase 1 local development, if `TPT__GOV__JWT_SECRET` is unset the
//! middleware logs a warning and allows requests through unauthenticated.

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::state::AppState;

/// Claims carried in a citizen access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the citizen DID (`did:gov:nz:...`).
    pub sub: String,
    /// Expiry (seconds since epoch).
    pub exp: usize,
    /// Consent scopes granted to this token, e.g. `ird:income`.
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// JWT verification configuration.
pub struct JwtConfig {
    decoding: Option<DecodingKey>,
    validation: Validation,
    enabled: bool,
}

impl JwtConfig {
    /// Build from environment.
    ///
    /// - `TPT__GOV__JWT_SECRET` — HMAC secret (enables enforcement when set).
    /// - `TPT__GOV__JWT_AUDIENCE` — optional expected audience claim.
    pub fn from_env() -> Self {
        match std::env::var("TPT__GOV__JWT_SECRET") {
            Ok(secret) if !secret.is_empty() => {
                let mut validation = Validation::new(Algorithm::HS256);
                validation.validate_exp = true;
                if let Ok(aud) = std::env::var("TPT__GOV__JWT_AUDIENCE") {
                    if !aud.is_empty() {
                        validation.set_audience(&[aud]);
                    }
                }
                Self {
                    decoding: Some(DecodingKey::from_secret(secret.as_bytes())),
                    validation,
                    enabled: true,
                }
            }
            _ => {
                tracing::warn!(
                    "TPT__GOV__JWT_SECRET not set — JWT auth disabled (dev mode; all requests allowed)"
                );
                Self {
                    decoding: None,
                    validation: Validation::new(Algorithm::HS256),
                    enabled: false,
                }
            }
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    fn decode_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let key = self
            .decoding
            .as_ref()
            .expect("decode_token called while JWT auth disabled");
        Ok(decode::<Claims>(token, key, &self.validation)?.claims)
    }
}

/// Axum middleware enforcing JWT auth on protected routes.
pub async fn jwt_auth(State(app): State<AppState>, mut req: Request, next: Next) -> Response {
    if !app.jwt.enabled() {
        return next.run(req).await;
    }

    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ").or_else(|| s.strip_prefix("bearer ")))
        .map(str::trim);

    let token = match token {
        Some(t) if !t.is_empty() => t,
        _ => return unauthorized("missing or malformed Authorization bearer token"),
    };

    match app.jwt.decode_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            next.run(req).await
        }
        Err(e) => unauthorized(&format!("invalid token: {e}")),
    }
}

fn unauthorized(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "unauthorized", "message": message })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    fn config_with_secret(secret: &str) -> JwtConfig {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        JwtConfig {
            decoding: Some(DecodingKey::from_secret(secret.as_bytes())),
            validation,
            enabled: true,
        }
    }

    fn make_token(secret: &str, exp: usize, scopes: Vec<String>) -> String {
        let claims = Claims {
            sub: "did:gov:nz:citizen001".into(),
            exp,
            scopes,
        };
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    #[test]
    fn decodes_valid_token() {
        let cfg = config_with_secret("test-secret");
        let exp = (chrono::Utc::now().timestamp() + 3600) as usize;
        let token = make_token("test-secret", exp, vec!["ird:income".into()]);

        let claims = cfg.decode_token(&token).unwrap();
        assert_eq!(claims.sub, "did:gov:nz:citizen001");
        assert_eq!(claims.scopes, vec!["ird:income".to_string()]);
    }

    #[test]
    fn rejects_wrong_secret() {
        let cfg = config_with_secret("test-secret");
        let exp = (chrono::Utc::now().timestamp() + 3600) as usize;
        let token = make_token("other-secret", exp, vec![]);
        assert!(cfg.decode_token(&token).is_err());
    }

    #[test]
    fn rejects_expired_token() {
        let cfg = config_with_secret("test-secret");
        // Well beyond jsonwebtoken's default 60s expiry leeway.
        let exp = (chrono::Utc::now().timestamp() - 300) as usize;
        let token = make_token("test-secret", exp, vec![]);
        assert!(cfg.decode_token(&token).is_err());
    }
}
