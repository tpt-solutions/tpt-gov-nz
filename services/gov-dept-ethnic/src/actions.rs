use axum::Json;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::EthnicError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, EthnicError> {
    let result = match action_type {
        "request-grant-info" => request_grant_info(pool, citizen_id, parameters).await,
        _ => Err(EthnicError::InvalidAction(format!(
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

async fn request_grant_info(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, EthnicError> {
    let grant_name = parameters
        .get("grantName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| EthnicError::InvalidAction("grantName must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "Grant info request received for", grant_name),
        "grantName": grant_name,
    }))
}
