use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsdError {
    #[error("Citizen not found")]
    CitizenNotFound,

    #[error("Scope not granted: {0}")]
    ScopeNotGranted(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid action: {0}")]
    InvalidAction(String),

    #[error("Action failed: {0}")]
    ActionFailed(String),
}

impl IntoResponse for MsdError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            MsdError::CitizenNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            MsdError::ScopeNotGranted(_) => (StatusCode::FORBIDDEN, self.to_string()),
            MsdError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_owned()),
            MsdError::InvalidAction(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            MsdError::ActionFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
