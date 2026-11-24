use std::net::SocketAddr;
use tracing::info;

use gov_dept_ird::build_app;
use gov_mtls::Transport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "gov_dept_ird=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8090".into())
        .parse()?;

    // Decide the transport before touching the database, so a node with no mTLS
    // configuration fails at startup rather than after serving starts.
    let transport = gov_mtls::transport_from_env()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run migrations on startup
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    let app = build_app(pool);
    let listener = tokio::net::TcpListener::bind(http_listen).await?;

    match transport {
        Transport::Mtls { config, allowed } => {
            info!(listen = %http_listen, dept = "ird", peers = allowed.len(), "gov-dept-ird starting over mTLS");
            gov_mtls::serve(listener, gov_mtls::restrict_to_peers(app, allowed), config).await?;
        }
        Transport::Development => {
            tracing::warn!(
                "TPT__GOV__ENV=development: serving plain HTTP with no client authentication"
            );
            info!(listen = %http_listen, dept = "ird", "gov-dept-ird starting");
            axum::serve(listener, app).await?;
        }
    }
    Ok(())
}
