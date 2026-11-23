use axum::Json;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::CaaError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, CaaError> {
    let result = match action_type {
        "request-licence-replacement" => {
            request_licence_replacement(pool, citizen_id, parameters).await
        }
        _ => Err(CaaError::InvalidAction(format!(
            "Unknown action: {action_type}"
        ))),
    };

    let (success, message) = match &result {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    db::log_action(
        pool,
        db::ActionLogEntry {
            citizen_id,
            action_type,
            parameters: parameters.clone(),
            performed_by,
            ai_level,
            result_success: success,
            result_message: message.as_deref(),
        },
    )
    .await?;

    result.map(Json)
}

async fn request_licence_replacement(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, CaaError> {
    let licence_no = parameters
        .get("licenceNo")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| CaaError::InvalidAction("licenceNo must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "Licence replacement request received for", licence_no),
        "licenceNo": licence_no,
    }))
}
