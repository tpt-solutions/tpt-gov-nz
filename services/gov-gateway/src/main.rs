use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "gov_gateway=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let addr: SocketAddr = std::env::var("TPT__GOV__GATEWAY_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    // Build dept service registry from env vars.
    // Pattern: TPT__GOV__DEPT_IRD_URL=http://localhost:8090
    let dept_registry = routes::DeptRegistry::from_env();

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/v1/citizen/resolve", axum::routing::post(routes::citizen_resolve))
        .route("/v1/dept/:dept_id/{*path}", axum::routing::any(routes::proxy_dept))
        .with_state(dept_registry)
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-gateway starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
