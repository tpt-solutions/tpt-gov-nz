use axum::{Router, routing::get};
use gov_federation_node::{FederationNode, FederationNodeConfig};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod audit;
mod consent;
mod opa;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_dept_node=info".into()))
        .json()
        .init();

    let dept_id = std::env::var("TPT__GOV__DEPT_ID")?;
    let database_url = std::env::var("DATABASE_URL")?;
    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8090".into())
        .parse()?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Ensure the audit log table exists before serving traffic.
    crate::audit::ensure_schema(&pool).await?;

    let fed_config = FederationNodeConfig::from_env()?;
    let fed_node = FederationNode::new(fed_config);
    fed_node.start().await?;

    let app = Router::new()
        .route("/health", get(routes::health))
        .route(
            "/citizen/resolve",
            axum::routing::post(routes::resolve_citizen),
        )
        .route("/citizen/data", axum::routing::post(routes::fetch_data))
        .route(
            "/citizen/action",
            axum::routing::post(routes::submit_action),
        )
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    info!(dept_id = %dept_id, listen = %http_listen, "gov-dept-node starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
