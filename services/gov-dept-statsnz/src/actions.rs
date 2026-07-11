use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::StatsnzError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, StatsnzError> {
    let result = match action_type {
        "request-data-export" => request_data_export(pool, citizen_id, parameters).await,
        _ => Err(StatsnzError::InvalidAction(format!(
            "Unknown action: {action_type}"
        ))),
    };

    let (success, message) = match &result {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    db::log_action(
        pool,
        citizen_id,
        action_type,
        parameters.clone(),
        performed_by,
        ai_level,
        success,
        message.as_deref(),
    )
    .await?;

    result.map(Json)
}

async fn request_data_export(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, StatsnzError> {
    let purpose = parameters
        .get("purpose")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| StatsnzError::InvalidAction("purpose must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Data export request received. We will process your export for: {purpose}"),
        "purpose": purpose,
    }))
}
