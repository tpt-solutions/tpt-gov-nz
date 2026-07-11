use axum::Json;
use serde_json::{json, Value};
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
        "request-grant-info" => requestGrantInfo(pool, citizen_id, parameters).await,
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

async fn requestGrantInfo(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, EthnicError> {
    let grantName = parameters
        .get("grantName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| EthnicError::InvalidAction("grantName must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "Grant info request received for", grantName),
        "grantName": grantName,
    }))
}
