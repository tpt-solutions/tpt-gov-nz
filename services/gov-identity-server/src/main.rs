use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use gov_identity_core::CredentialIssuer;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

mod auth;
mod jwt;
mod routes;
mod state;

use jwt::jwt_secret_from_env;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "gov_identity_server=info".into()),
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

    // JWT secret for portal/dept service authentication (optional for dev).
    let jwt_secret = jwt_secret_from_env();
    if jwt_secret.is_none() {
        warn!("TPT__GOV__JWT_SECRET not set — JWT authentication DISABLED (dev mode only)");
    }

    let app_state = AppState::new(pool.clone(), issuer, api_key, jwt_secret);

    // Spawn background job for grant TTL purge and revocation cleanup.
    let purge_pool = pool.clone();
    let purge_state = app_state.clone();
    tokio::spawn(async move {
        run_purge_job(purge_pool, purge_state).await;
    });

    // Public endpoints: health, issuer key discovery, JWKS, OIDC discovery, and DID lookup.
    let public = Router::new()
        .route("/health", get(routes::health))
        .route("/v1/issuer/public-key", get(routes::issuer_public_key))
        .route("/.well-known/jwks.json", get(routes::jwks))
        .route(
            "/.well-known/openid-configuration",
            get(routes::oidc_discovery),
        )
        .route("/v1/did/{did}", get(routes::get_did_document));

    // DID registration endpoints require API key (portals/dept services).
    let did_register = Router::new()
        .route("/v1/did/challenge", post(routes::did_challenge))
        .route("/v1/did/register", post(routes::register_did_with_proof))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::require_api_key,
        ));

    // JWT-authenticated endpoints: portals and dept services.
    let jwt_protected = Router::new()
        .route("/v1/grants", post(routes::issue_grant))
        .route("/v1/grants", get(routes::list_grants))
        .route("/v1/grants/status", post(routes::grant_status))
        .route("/v1/grants/{id}", delete(routes::revoke_grant))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt::require_jwt,
        ));

    // API-key authenticated admin endpoints.
    let admin = Router::new()
        .route("/v1/admin/grants", get(routes::admin_list_grants))
        .route(
            "/v1/admin/grants/{citizen_did}",
            delete(routes::admin_revoke_citizen_grants),
        )
        .route(
            "/v1/admin/issuer/rotate",
            post(routes::admin_rotate_issuer_key),
        )
        .route("/v1/admin/depts/health", get(routes::admin_dept_health))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::require_api_key,
        ));

    let app = Router::new()
        .merge(public)
        .merge(did_register)
        .merge(jwt_protected)
        .merge(admin)
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

/// Background job: periodically purge expired grants and clean up revocation list.
async fn run_purge_job(pool: PgPool, _state: AppState) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600)); // hourly
    loop {
        interval.tick().await;
        if let Err(e) = purge_expired_grants(&pool).await {
            tracing::error!(error = %e, "Failed to purge expired grants");
        }
        if let Err(e) = cleanup_revocation_list(&pool).await {
            tracing::error!(error = %e, "Failed to cleanup revocation list");
        }
        // Also clean up old rotation log entries (keep 1 year).
        if let Err(e) = cleanup_rotation_log(&pool).await {
            tracing::error!(error = %e, "Failed to cleanup rotation log");
        }
    }
}

/// Purge expired grants: mark them as revoked with reason 'ttl_expired'.
async fn purge_expired_grants(pool: &PgPool) -> anyhow::Result<()> {
    let now = chrono::Utc::now();
    let mut tx = pool.begin().await?;

    // Find grants that have expired but not yet revoked.
    let expired: Vec<uuid::Uuid> = sqlx::query_scalar(
        r#"
        SELECT id FROM data_grants
        WHERE expires_at < $1 AND revoked_at IS NULL
        "#,
    )
    .bind(now)
    .fetch_all(&mut *tx)
    .await?;

    for grant_id in &expired {
        // Mark grant as revoked.
        sqlx::query(
            r#"
            UPDATE data_grants SET revoked_at = $1 WHERE id = $2
            "#,
        )
        .bind(now)
        .bind(grant_id)
        .execute(&mut *tx)
        .await?;

        // Add to revocation list.
        sqlx::query(
            r#"
            INSERT INTO revocation_list (grant_id, citizen_did, revoked_at, revoked_by, reason)
            SELECT $1, citizen_did, $2, 'ttl_expired', 'Grant TTL expired'
            FROM data_grants WHERE id = $1
            "#,
        )
        .bind(grant_id)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    if !expired.is_empty() {
        info!(count = expired.len(), "Purged expired grants");
    }
    Ok(())
}

/// Clean up revocation list entries older than 7 years (regulatory retention).
async fn cleanup_revocation_list(pool: &PgPool) -> anyhow::Result<()> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(7 * 365);
    let deleted: u64 = sqlx::query("DELETE FROM revocation_list WHERE revoked_at < $1")
        .bind(cutoff)
        .execute(pool)
        .await?
        .rows_affected();

    if deleted > 0 {
        info!(deleted, "Cleaned up old revocation list entries");
    }
    Ok(())
}

/// Clean up rotation log entries older than 1 year.
async fn cleanup_rotation_log(pool: &PgPool) -> anyhow::Result<()> {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(365);
    let deleted: u64 = sqlx::query("DELETE FROM issuer_key_rotation_log WHERE rotated_at < $1")
        .bind(cutoff)
        .execute(pool)
        .await?
        .rows_affected();

    if deleted > 0 {
        info!(deleted, "Cleaned up old rotation log entries");
    }
    Ok(())
}
