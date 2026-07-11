use axum::{
    middleware,
    routing::{any, get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

mod auth;
mod circuit_breaker;
mod rate_limit;
mod routes;
mod state;
mod telemetry;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let _telemetry = telemetry::init()?;

    let addr: SocketAddr = std::env::var("TPT__GOV__GATEWAY_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    // Build dept service registry from env vars.
    // Pattern: TPT__GOV__DEPT_IRD_URL=http://localhost:8090
    let mtls = gov_mtls::TlsPaths::from_env().map(|paths| {
        let cfg = gov_mtls::client_config(&paths).expect("invalid mTLS client config");
        Arc::new(gov_mtls::MtlsClient::new(Arc::new(cfg)))
    });
    if mtls.is_some() {
        info!("mTLS enabled for upstream department calls");
    }

    let app_state = AppState {
        registry: routes::DeptRegistry::from_env(),
        rate_limiter: rate_limit::RateLimiter::from_env(),
        breakers: circuit_breaker::CircuitBreakerRegistry::from_env(),
        jwt: Arc::new(auth::JwtConfig::from_env()),
        mtls,
    };

    // Protected API surface. Middleware executes outermost-first:
    //   JWT auth -> rate limit -> circuit breaker -> handler
    let protected = Router::new()
        .route("/v1/citizen/resolve", post(routes::citizen_resolve))
        .route("/v1/dept/{dept_id}/{*path}", any(routes::proxy_dept))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            circuit_breaker::circuit_breaker,
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            rate_limit::rate_limit,
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::jwt_auth,
        ));

    let app = Router::new()
        .route("/health", get(routes::health))
        .merge(protected)
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-gateway starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
