use axum::{Router, routing::get};
use gov_federation_core::NodeKeypair;
use gov_federation_node::{FederationNode, FederationNodeConfig};
use std::net::SocketAddr;
use std::sync::Arc;
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

    // Load or create audit signing keypair from environment.
    let audit_keypair = Arc::new(load_or_create_audit_keypair(&dept_id).await?);

    let fed_config = FederationNodeConfig::from_env()?;
    let fed_node = FederationNode::new(fed_config);

    // Run federation node in background, HTTP server in foreground.
    let fed_node_handle = tokio::spawn(async move {
        if let Err(e) = fed_node.start().await {
            tracing::error!(error = %e, "Federation node stopped");
        }
    });

    // Also start QUIC transport if enabled
    #[cfg(feature = "quic")]
    {
        let fed_node_quic = fed_node.clone();
        tokio::spawn(async move {
            if let Err(e) = fed_node_quic.start_quic().await {
                tracing::error!(error = %e, "QUIC federation server stopped");
            }
        });
    }

    let state = routes::AppState {
        pool,
        audit_keypair,
    };

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
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    info!(dept_id = %dept_id, listen = %http_listen, "gov-dept-node starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;

    // Wait for federation node to finish (should not happen in normal operation)
    fed_node_handle.await?;
    Ok(())
}

/// Load the audit signing keypair from environment, or generate a persistent one.
async fn load_or_create_audit_keypair(dept_id: &str) -> anyhow::Result<NodeKeypair> {
    // Try to load from environment first
    if let Ok(secret_b64) = std::env::var("TPT__GOV__AUDIT_SECRET_KEY") {
        return NodeKeypair::from_secret_b64(&secret_b64)
            .map_err(|e| anyhow::anyhow!("invalid audit secret key: {e}"));
    }

    // Generate a new one (in production, this should be stored securely)
    let keypair = NodeKeypair::generate(dept_id);
    tracing::warn!(
        "Generated ephemeral audit keypair for {}. Set TPT__GOV__AUDIT_SECRET_KEY for production.",
        dept_id
    );
    Ok(keypair)
}
