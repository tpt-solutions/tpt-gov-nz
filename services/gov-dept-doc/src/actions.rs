use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::DocError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, DocError> {
    let result = match action_type {
        "apply-conservation-permit" => apply_conservation_permit(pool, citizen_id, parameters).await,
        _ => Err(DocError::InvalidAction(format!(
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

async fn apply_conservation_permit(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, DocError> {
    let activity = parameters
        .get("activity")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| DocError::InvalidAction("activity must not be empty".to_owned()))?;

    let location = parameters
        .get("location")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| DocError::InvalidAction("location must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Conservation permit application for '{activity}' at {location} received. We will assess your application and be in touch."),
        "activity": activity,
        "location": location,
    }))
}
