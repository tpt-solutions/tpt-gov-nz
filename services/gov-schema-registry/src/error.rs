use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("schema not found")]
    NotFound,
    #[error("unauthorized: missing or invalid X-Registry-Key")]
    Unauthorized,
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl IntoResponse for RegistryError {
    fn into_response(self) -> Response {
        let status = match self {
            RegistryError::NotFound => StatusCode::NOT_FOUND,
            RegistryError::Unauthorized => StatusCode::UNAUTHORIZED,
            RegistryError::Invalid(_) => StatusCode::BAD_REQUEST,
            RegistryError::Db(_) | RegistryError::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
