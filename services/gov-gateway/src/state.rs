use std::sync::Arc;

use axum::extract::FromRef;

use crate::auth::JwtConfig;
use crate::circuit_breaker::CircuitBreakerRegistry;
use crate::rate_limit::RateLimiter;
use crate::routes::DeptRegistry;

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
}

impl FromRef<AppState> for DeptRegistry {
    fn from_ref(state: &AppState) -> Self {
        state.registry.clone()
    }
}
