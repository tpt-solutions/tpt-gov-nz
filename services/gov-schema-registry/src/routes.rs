use axum::{
    extract::{Path, State},
    http::HeaderMap,
    http::StatusCode,
    Json,
};
use serde_json::Value;
use tower_http::trace::TraceLayer;

use crate::{
    db::{self, SchemaRow},
    error::RegistryError,
    validate,
};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn list_schemas(State(pool): State<sqlx::PgPool>) -> Result<Json<Vec<SchemaRow>>, RegistryError> {
    Ok(Json(db::list_latest(&pool).await?))
}

pub async fn get_latest(
    State(pool): State<sqlx::PgPool>,
    Path(name): Path<String>,
) -> Result<Json<SchemaRow>, RegistryError> {
    db::get_latest(&pool, &name)
        .await
        .map(Json)
        .map_err(|_| RegistryError::NotFound)
}

pub async fn get_version(
    State(pool): State<sqlx::PgPool>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<SchemaRow>, RegistryError> {
    db::get_version(&pool, &name, &version)
        .await
        .map(Json)
        .map_err(|_| RegistryError::NotFound)
}

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub content: Value,
}

pub async fn register_schema(
    headers: HeaderMap,
    State(pool): State<sqlx::PgPool>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<SchemaRow>), RegistryError> {
    let expected = std::env::var("SCHEMA_REGISTRY_KEY").unwrap_or_default();
    let provided = headers
        .get("x-registry-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if expected.is_empty() || provided != expected {
        return Err(RegistryError::Unauthorized);
    }

    if body.name.trim().is_empty() || body.version.trim().is_empty() {
        return Err(RegistryError::Invalid(
            "name and version are required".into(),
        ));
    }

    let row = db::register(&pool, &body.name, &body.version, &body.content).await?;
    Ok((StatusCode::CREATED, Json(row)))
}

#[derive(serde::Deserialize)]
pub struct ValidateRequest {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    pub payload: Value,
}

pub async fn validate_handler(
    State(pool): State<sqlx::PgPool>,
    Json(body): Json<ValidateRequest>,
) -> Result<Json<serde_json::Value>, RegistryError> {
    let row = match &body.version {
        Some(v) => db::get_version(&pool, &body.name, v).await,
        None => db::get_latest(&pool, &body.name).await,
    }
    .map_err(|_| RegistryError::NotFound)?;

    match validate::validate(&row.content, &body.payload) {
        Ok(()) => Ok(Json(serde_json::json!({
            "valid": true,
            "name": row.name,
            "version": row.version,
        }))),
        Err(reason) => Ok(Json(serde_json::json!({
            "valid": false,
            "name": row.name,
            "version": row.version,
            "error": reason,
        }))),
    }
}

/// Build the schema-registry router.
pub fn build_app(pool: sqlx::PgPool) -> axum::Router {
    axum::Router::new()
        .route("/health", axum::routing::get(health))
        .route(
            "/v1/schemas",
            axum::routing::get(list_schemas).post(register_schema),
        )
        .route("/v1/schemas/:name", axum::routing::get(get_latest))
        .route(
            "/v1/schemas/:name/:version",
            axum::routing::get(get_version),
        )
        .route("/v1/validate", axum::routing::post(validate_handler))
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
}
