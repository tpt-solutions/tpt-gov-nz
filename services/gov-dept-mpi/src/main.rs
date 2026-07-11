use std::net::SocketAddr;
use tracing::info;

use gov_dept_mpi::build_app;
use gov_federation_node::{FederationNode, FederationNodeConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_dept_mpi=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8106".into())
        .parse()?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    let fed_config = FederationNodeConfig::from_env().unwrap_or_else(|_| FederationNodeConfig {
        dept_id: "mpi".into(),
        listen_addr: "0.0.0.0:7017".parse().expect("valid federation addr"),
        peers: vec![],
    });
    let fed_node = FederationNode::new(fed_config);
    let fed_public_key = fed_node.public_key_b64();
    tokio::spawn(async move {
        if let Err(e) = fed_node.start().await {
            tracing::error!(error = %e, "Federation node stopped");
        }
    });

    let app = build_app(pool);

    info!(listen = %http_listen, dept = "mpi", federation_public_key = %fed_public_key, "gov-dept-mpi starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
