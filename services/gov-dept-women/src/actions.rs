use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::WomenError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, WomenError> {
    let result = match action_type {
        "request-programme-info" => requestProgrammeInfo(pool, citizen_id, parameters).await,
        _ => Err(WomenError::InvalidAction(format!(
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

async fn requestProgrammeInfo(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, WomenError> {
    let programmeName = parameters
        .get("programmeName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| WomenError::InvalidAction("programmeName must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "Programme info request received for", programmeName),
        "programmeName": programmeName,
    }))
}
