//! Shared state for the identity server.

use std::sync::Arc;

use gov_identity_core::CredentialIssuer;
use sqlx::PgPool;
use tokio::sync::broadcast;

/// Shared application state.
///
/// The issuer keypair is loaded once at startup and shared across requests, so
/// every credential is signed by the same persistent key.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub issuer: Arc<CredentialIssuer>,
    pub api_key: Arc<str>,
    pub jwt_secret: Option<Arc<str>>,
    pub _jwt_audience: String,
    /// Broadcast channel for signalling issuer key rotation to background tasks.
    pub rotation_tx: broadcast::Sender<RotationEvent>,
}

/// Event emitted when the issuer key is rotated.
#[derive(Debug, Clone)]
pub struct RotationEvent {
    pub _old_did: String,
    pub _new_did: String,
    pub _rotated_at: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        issuer: CredentialIssuer,
        api_key: Arc<str>,
        jwt_secret: Option<Arc<str>>,
    ) -> Self {
        let _jwt_audience =
            std::env::var("TPT__GOV__JWT_AUDIENCE").unwrap_or_else(|_| "tpt-gov-nz".into());
        let (rotation_tx, _) = broadcast::channel(16);
        Self {
            pool,
            issuer: Arc::new(issuer),
            api_key,
            jwt_secret,
            _jwt_audience,
            rotation_tx,
        }
    }
}
