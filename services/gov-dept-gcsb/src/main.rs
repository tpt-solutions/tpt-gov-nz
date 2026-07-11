use std::net::SocketAddr;
use tracing::info;

use gov_dept_gcsb::build_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_dept_gcsb=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8146".into())
        .parse()?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    let app = build_app(pool);

    info!(listen = %http_listen, dept = "gcsb", "gov-dept-gcsb starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
