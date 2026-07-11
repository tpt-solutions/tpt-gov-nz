use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::LinzError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, LinzError> {
    let result = match action_type {
        "request-title-copy" => request_title_copy(pool, citizen_id, parameters).await,
        "update-mailing-address" => update_mailing_address(pool, citizen_id, parameters).await,
        _ => Err(LinzError::InvalidAction(format!(
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

async fn request_title_copy(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, LinzError> {
    let title_number = parameters
        .get("titleNumber")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| LinzError::InvalidAction("titleNumber is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Title copy request for {title_number} received. LINZ will post a certified copy to your registered address."),
        "titleNumber": title_number,
    }))
}

async fn update_mailing_address(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, LinzError> {
    let address = parameters
        .get("address")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| LinzError::InvalidAction("address is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Mailing address updated to: {address}. Updates may take up to 10 working days to appear on titles."),
        "address": address,
    }))
}
