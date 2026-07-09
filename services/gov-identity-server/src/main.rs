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
                .unwrap_or_else(|_| "gov_identity_server=info".into()),
        )
        .json()
        .init();

    let addr: SocketAddr = std::env::var("TPT__GOV__IDENTITY_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8081".into())
        .parse()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/v1/did/register", axum::routing::post(routes::register_did))
        .route("/v1/did/:did", get(routes::get_did_document))
        .route("/v1/grants", axum::routing::post(routes::issue_grant))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-identity-server starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
