//! Shared state for the identity server.

use std::sync::Arc;

use gov_identity_core::CredentialIssuer;
use sqlx::PgPool;

/// Shared application state.
///
/// The issuer keypair is loaded once at startup and shared across requests, so
/// every credential is signed by the same persistent key.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub issuer: Arc<CredentialIssuer>,
    pub api_key: Arc<str>,
}

impl AppState {
    pub fn new(pool: PgPool, issuer: CredentialIssuer, api_key: Arc<str>) -> Self {
        Self {
            pool,
            issuer: Arc::new(issuer),
            api_key,
        }
    }
}
