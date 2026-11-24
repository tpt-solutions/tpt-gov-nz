//! JWT validation middleware for the API gateway.
//!
//! Validates a `Bearer` token on protected routes using HS256 and the shared
//! secret in `TPT__GOV__JWT_SECRET`. On success the decoded [`Claims`] are
//! inserted into request extensions so downstream handlers and middleware can
//! read the citizen DID and granted scopes.
//!
//! The gateway **fails closed**: it refuses to start when `TPT__GOV__JWT_SECRET`
//! is unset or too short. There is deliberately no "auth disabled" mode — local
//! development sets the secret in `.env` (see `.env.example`).

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

use crate::state::AppState;

/// Minimum accepted HMAC secret length.
const MIN_SECRET_LEN: usize = 16;

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
    /// Token type. `citizen` marks a token minted for the citizen portal, where the
    /// subject is acting on their own data. Absent for other callers.
    #[serde(default)]
    pub typ: Option<String>,
}

impl Claims {
    /// True when this token grants at least one scope for `dept_id`.
    pub fn permits_dept(&self, dept_id: &str) -> bool {
        let prefix = format!("{dept_id}:");
        self.scopes.iter().any(|s| s.starts_with(&prefix))
    }
}

/// JWT verification configuration.
pub struct JwtConfig {
    decoding: DecodingKey,
    validation: Validation,
}

impl JwtConfig {
    /// Build from environment.
    ///
    /// - `TPT__GOV__JWT_SECRET` — required HMAC secret.
    /// - `TPT__GOV__JWT_AUDIENCE` — optional expected audience claim.
    ///
    /// Returns an error (so the process refuses to start) when the secret is
    /// missing or shorter than [`MIN_SECRET_LEN`] bytes.
    pub fn from_env() -> anyhow::Result<Self> {
        let secret = std::env::var("TPT__GOV__JWT_SECRET").map_err(|_| {
            anyhow::anyhow!(
                "TPT__GOV__JWT_SECRET is required — the gateway will not start with \
                 authentication disabled (set it in .env; see .env.example)"
            )
        })?;
        if secret.len() < MIN_SECRET_LEN {
            anyhow::bail!("TPT__GOV__JWT_SECRET must be at least {MIN_SECRET_LEN} characters");
        }

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        if let Ok(aud) = std::env::var("TPT__GOV__JWT_AUDIENCE")
            && !aud.is_empty()
        {
            validation.set_audience(&[aud]);
        }

        Ok(Self {
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            validation,
        })
    }

    fn decode_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        Ok(decode::<Claims>(token, &self.decoding, &self.validation)?.claims)
    }
}

/// Axum middleware enforcing JWT auth on protected routes.
pub async fn jwt_auth(State(app): State<AppState>, mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.strip_prefix("Bearer ")
                .or_else(|| s.strip_prefix("bearer "))
        })
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
        Err(e) => {
            // Log the detailed reason; never echo token-parsing internals to the
            // caller (they can aid token forgery attempts).
            tracing::warn!(error = %e, "rejected invalid bearer token");
            unauthorized("invalid or expired bearer token")
        }
    }
}

fn unauthorized(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, "Bearer")],
        Json(json!({ "error": "unauthorized", "message": message })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{EncodingKey, Header, encode};

    fn config_with_secret(secret: &str) -> JwtConfig {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        JwtConfig {
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            validation,
        }
    }

    fn make_token(secret: &str, exp: usize, scopes: Vec<String>) -> String {
        let claims = Claims {
            sub: "did:gov:nz:citizen001".into(),
            exp,
            scopes,
            typ: None,
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

    #[test]
    fn scope_check_is_per_department() {
        let claims = Claims {
            sub: "did:gov:nz:citizen001".into(),
            exp: 0,
            scopes: vec!["ird:income".into(), "winz:benefit-status".into()],
            typ: None,
        };
        assert!(claims.permits_dept("ird"));
        assert!(claims.permits_dept("winz"));
        assert!(!claims.permits_dept("moh"));
        // A scope for a different dept must not leak across.
        assert!(!claims.permits_dept("ir"));
    }
}
