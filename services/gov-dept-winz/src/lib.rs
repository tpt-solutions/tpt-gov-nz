use axum::{routing::get, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

mod actions;
mod consent;
mod db;
mod error;
mod opa;
mod routes;

pub use error::WinzError;

/// Build the WINZ department service router with the given connection pool as state.
/// Shared by `main` (production) and the integration tests (HTTP round-trip).
pub fn build_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/citizen/resolve", axum::routing::post(routes::resolve_citizen))
        .route("/citizen/data", axum::routing::post(routes::fetch_data))
        .route("/citizen/action", axum::routing::post(routes::submit_action))
        .route("/citizen/{did}/benefits", get(routes::list_benefits))
        .route("/citizen/{did}/payments", get(routes::list_payments))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests;
