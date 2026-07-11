use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MbieError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MbieError> {
    let result = match action_type {
        "register-business" => register_business(pool, citizen_id, parameters).await,
        "update-director-details" => update_director_details(pool, citizen_id, parameters).await,
        _ => Err(MbieError::InvalidAction(format!(
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

async fn register_business(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MbieError> {
    let nzbn = parameters
        .get("nzbn")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MbieError::InvalidAction("nzbn is required".to_owned()))?;

    let entity_name = parameters
        .get("entityName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MbieError::InvalidAction("entityName is required".to_owned()))?;

    let entity_type = parameters
        .get("entityType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MbieError::InvalidAction("entityType is required".to_owned()))?;

    if !matches!(
        entity_type,
        "company" | "sole-trader" | "partnership" | "trust"
    ) {
        return Err(MbieError::InvalidAction(format!(
            "entityType must be one of company, sole-trader, partnership, trust (got '{entity_type}')"
        )));
    }

    Ok(json!({
        "success": true,
        "message": format!("Business registration ({entity_name}) submitted. We will assess your registration and be in touch."),
        "nzbn": nzbn,
        "entityName": entity_name,
        "entityType": entity_type,
    }))
}

async fn update_director_details(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MbieError> {
    let address = parameters
        .get("address")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MbieError::InvalidAction("address must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": "Director details updated. Your registered address has been recorded.",
        "address": address,
    }))
}
