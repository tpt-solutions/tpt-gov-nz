use std::sync::Arc;
use std::time::Duration;

use axum::{body::Bytes, extract::FromRef};
use http_body_util::Full;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

use crate::auth::JwtConfig;
use crate::circuit_breaker::CircuitBreakerRegistry;
use crate::rate_limit::RateLimiter;
use crate::routes::DeptRegistry;

/// Shared HTTP client used for all upstream calls.
///
/// A single pooled client is reused for every request so the gateway keeps
/// connections alive to department services instead of opening a new
/// connection per request.
pub type SharedHttpClient = Client<HttpConnector, Full<Bytes>>;

/// Shared application state for the gateway.
///
/// Handlers that only need the department registry continue to extract
/// `State<DeptRegistry>` thanks to the [`FromRef`] impl below; middleware
/// extracts the full `State<AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub registry: DeptRegistry,
    pub rate_limiter: RateLimiter,
    pub breakers: CircuitBreakerRegistry,
    pub jwt: Arc<JwtConfig>,
    /// Optional mutual-TLS client used to reach internal department services.
    /// Built from `TPT__GOV__MTLS_*` env when present; `None` disables mTLS and
    /// uses plaintext HTTP (demo / untrusted networks only).
    pub mtls: Option<Arc<gov_mtls::MtlsClient>>,
    /// Pooled client for plaintext upstream calls (identity server + depts).
    pub http: SharedHttpClient,
    /// Base URL of the identity server, used by `citizen_resolve`.
    pub identity_url: String,
    /// Timeout applied to every upstream request.
    pub upstream_timeout: Duration,
    /// Maximum accepted request body size for proxied department calls.
    pub max_body_bytes: usize,
}

impl AppState {
    /// Build the shared upstream client with a bounded idle pool.
    pub fn build_http_client() -> SharedHttpClient {
        Client::builder(hyper_util::rt::TokioExecutor::new())
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(32)
            .build_http()
    }
}

impl FromRef<AppState> for DeptRegistry {
    fn from_ref(state: &AppState) -> Self {
        state.registry.clone()
    }
}
