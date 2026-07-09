use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod actions;
mod db;
mod error;
mod routes;

pub use error::IrdError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_dept_ird=info,tower_http=debug".into()),
        )
        .json()
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let http_listen: SocketAddr = std::env::var("TPT__GOV__HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8090".into())
        .parse()?;

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run migrations on startup
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/citizen/resolve", axum::routing::post(routes::resolve_citizen))
        .route("/citizen/data", axum::routing::post(routes::fetch_data))
        .route("/citizen/action", axum::routing::post(routes::submit_action))
        .route("/citizen/:did/tax-years", get(routes::list_tax_years))
        .route("/citizen/:did/gst-periods", get(routes::list_gst_periods))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    info!(listen = %http_listen, dept = "ird", "gov-dept-ird starting");
    let listener = tokio::net::TcpListener::bind(http_listen).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
