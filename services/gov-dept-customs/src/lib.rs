use axum::{routing::get, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

mod actions;
mod consent;
mod db;
mod error;
mod opa;
mod routes;

pub use error::CustomsError;

/// Build the Customs department service router with the given connection pool as state.
pub fn build_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/citizen/resolve", axum::routing::post(routes::resolve_citizen))
        .route("/citizen/data", axum::routing::post(routes::fetch_data))
        .route("/citizen/action", axum::routing::post(routes::submit_action))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests;
