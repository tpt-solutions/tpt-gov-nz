use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::HudError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, HudError> {
    let result = match action_type {
        "submit-housing-application" => submit_housing_application(pool, citizen_id, parameters).await,
        "request-maintenance" => request_maintenance(pool, citizen_id, parameters).await,
        _ => Err(HudError::InvalidAction(format!(
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

async fn submit_housing_application(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, HudError> {
    let application_type = parameters
        .get("applicationType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| HudError::InvalidAction("applicationType is required".to_owned()))?;

    if !matches!(
        application_type,
        "public-housing" | "emergency-housing" | "home-ownership"
    ) {
        return Err(HudError::InvalidAction(format!(
            "applicationType must be one of public-housing, emergency-housing, home-ownership (got '{application_type}')"
        )));
    }

    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| HudError::InvalidAction("reason must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Housing application ({application_type}) submitted. We will assess your application and be in touch."),
        "applicationType": application_type,
        "reason": reason,
    }))
}

async fn request_maintenance(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, HudError> {
    let category = parameters
        .get("category")
        .and_then(|v| v.as_str())
        .ok_or_else(|| HudError::InvalidAction("category is required".to_owned()))?;

    if !matches!(
        category,
        "plumbing" | "electrical" | "heating" | "structural" | "other"
    ) {
        return Err(HudError::InvalidAction(format!(
            "category must be one of plumbing, electrical, heating, structural, other (got '{category}')"
        )));
    }

    let description = parameters
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| HudError::InvalidAction("description must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Maintenance request ({category}) logged. A contractor will be in touch to schedule a visit."),
        "category": category,
        "description": description,
    }))
}
