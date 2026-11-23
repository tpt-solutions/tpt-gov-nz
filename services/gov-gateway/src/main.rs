use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{any, get, post},
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::info;

use anyhow::Context;

mod auth;
mod circuit_breaker;
mod consent;
mod metrics;
mod rate_limit;
mod readiness;
mod routes;
mod state;
mod telemetry;

use consent::consent_middleware;
use state::AppState;

/// Default maximum proxied request/response body size (1 MiB).
const DEFAULT_MAX_BODY_BYTES: usize = 1024 * 1024;
/// Default upstream request timeout (seconds).
const DEFAULT_UPSTREAM_TIMEOUT_SECS: u64 = 10;

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

    let identity_url = std::env::var("TPT__GOV__IDENTITY_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8081".into());
    let upstream_timeout = Duration::from_secs(
        std::env::var("TPT__GOV__UPSTREAM_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_UPSTREAM_TIMEOUT_SECS),
    );
    let max_body_bytes = std::env::var("TPT__GOV__MAX_BODY_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MAX_BODY_BYTES);

    // Fails closed: refuses to start without a valid JWT secret.
    let jwt = auth::JwtConfig::from_env()?;

    // Initialize consent configuration (OPA policies, grant verification, revocation).
    let consent = Arc::new(
        consent::ConsentConfig::from_env()
            .await
            .context("building consent configuration")?,
    );

    let app_state = AppState {
        registry: routes::DeptRegistry::from_env(),
        rate_limiter: rate_limit::RateLimiter::from_env(),
        breakers: circuit_breaker::CircuitBreakerRegistry::from_env(),
        jwt: Arc::new(jwt),
        consent,
        mtls,
        http: AppState::build_http_client(),
        identity_url,
        upstream_timeout,
        max_body_bytes,
    };

    // Protected API surface. Middleware executes outermost-first:
    //   JWT auth -> rate limit -> circuit breaker -> consent -> handler
    let protected = Router::new()
        .route("/v1/citizen/resolve", post(routes::citizen_resolve))
        .route("/v1/dept/{dept_id}/{*path}", any(routes::proxy_dept))
        .layer(DefaultBodyLimit::max(max_body_bytes))
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
            consent_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::jwt_auth,
        ));

    // Liveness (/health) never calls downstreams; readiness (/ready) probes them.
    // /health keeps its original response, which lists the registered departments.
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/ready", get(readiness::ready))
        .route("/metrics", get(metrics::endpoint))
        .merge(protected)
        .with_state(app_state)
        .layer(middleware::from_fn(metrics::track))
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-gateway starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
