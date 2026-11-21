use std::net::SocketAddr;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use gov_identity_core::CredentialIssuer;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::info;

mod auth;
mod routes;
mod state;

use state::AppState;

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

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations applied");

    // Load the persistent issuer key (env override, else generated once and
    // stored in the database) so credentials verify across restarts.
    let issuer = load_or_create_issuer(&pool).await?;
    info!(issuer_did = %issuer.did, "Issuer key loaded");

    // The identity server refuses to start without an API key.
    let api_key = auth::key_from_env()?;

    let app_state = AppState::new(pool, issuer, api_key);

    // Public endpoints: health, issuer key discovery, and DID lookup.
    let public = Router::new()
        .route("/health", get(routes::health))
        .route("/v1/issuer/public-key", get(routes::issuer_public_key))
        .route("/v1/did/{did}", get(routes::get_did_document));

    // Authenticated endpoints: all mutations and grant reads.
    let protected = Router::new()
        .route("/v1/did/register", post(routes::register_did))
        .route("/v1/grants", post(routes::issue_grant))
        .route("/v1/grants", get(routes::list_grants))
        .route("/v1/grants/status", post(routes::grant_status))
        .route("/v1/grants/{id}", delete(routes::revoke_grant))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::require_api_key,
        ));

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    info!(listen = %addr, "gov-identity-server starting");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Load the issuer key from `TPT__GOV__ISSUER_KEY` when set, otherwise from the
/// `issuer_keys` table, generating and persisting a new key on first start.
async fn load_or_create_issuer(pool: &PgPool) -> anyhow::Result<CredentialIssuer> {
    if let Ok(encoded) = std::env::var("TPT__GOV__ISSUER_KEY") {
        if !encoded.trim().is_empty() {
            info!("Using issuer key from TPT__GOV__ISSUER_KEY");
            return Ok(CredentialIssuer::from_secret_b64(&encoded)?);
        }
    }

    let existing: Option<String> =
        sqlx::query_scalar("SELECT secret_key_b64 FROM issuer_keys WHERE id = 1")
            .fetch_optional(pool)
            .await?;
    if let Some(encoded) = existing {
        return Ok(CredentialIssuer::from_secret_b64(&encoded)?);
    }

    let issuer = CredentialIssuer::generate();
    // Another instance may have won the race; keep whichever key is stored.
    sqlx::query(
        "INSERT INTO issuer_keys (id, secret_key_b64) VALUES (1, $1) ON CONFLICT (id) DO NOTHING",
    )
    .bind(issuer.secret_key_b64())
    .execute(pool)
    .await?;

    let stored: String = sqlx::query_scalar("SELECT secret_key_b64 FROM issuer_keys WHERE id = 1")
        .fetch_one(pool)
        .await?;
    Ok(CredentialIssuer::from_secret_b64(&stored)?)
}
