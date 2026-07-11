use sqlx::PgPool;

mod db;
mod error;
mod routes;
mod validate;

pub use db::SchemaRow;
pub use error::RegistryError;

/// Build the schema-registry router with the given connection pool as state.
pub fn build_app(pool: PgPool) -> axum::Router {
    routes::build_app(pool)
}
