use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::TpkError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, TpkError> {
    let result = match action_type {
        "apply-funding" => apply_funding(pool, citizen_id, parameters).await,
        _ => Err(TpkError::InvalidAction(format!(
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

async fn apply_funding(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, TpkError> {
    let programme = parameters
        .get("programme")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| TpkError::InvalidAction("programme is required".to_owned()))?;

    let purpose = parameters
        .get("purpose")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| TpkError::InvalidAction("purpose is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Funding application for programme '{programme}' submitted. We will assess your application and be in touch."),
        "programme": programme,
        "purpose": purpose,
    }))
}
