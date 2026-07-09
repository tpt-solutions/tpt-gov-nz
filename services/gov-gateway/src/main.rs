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

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/v1/citizen/resolve", axum::routing::post(routes::citizen_resolve))
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-gateway starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
