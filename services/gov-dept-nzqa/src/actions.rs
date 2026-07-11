use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::NzqaError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, NzqaError> {
    let result = match action_type {
        "request-transcript" => request_transcript(pool, citizen_id, parameters).await,
        "order-convocation" => order_convocation(pool, citizen_id, parameters).await,
        _ => Err(NzqaError::InvalidAction(format!(
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

async fn request_transcript(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, NzqaError> {
    let purpose = parameters
        .get("purpose")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| NzqaError::InvalidAction("purpose must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Transcript request received. We will prepare your Record of Achievement for: {purpose}."),
        "purpose": purpose,
    }))
}

async fn order_convocation(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, NzqaError> {
    let qualification_id = parameters
        .get("qualificationId")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| NzqaError::InvalidAction("qualificationId must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Convocation order placed for qualification {qualification_id}. A parchment will be dispatched."),
        "qualificationId": qualification_id,
    }))
}
