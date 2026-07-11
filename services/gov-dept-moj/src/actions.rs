use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MojError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MojError> {
    let result = match action_type {
        "pay-fine" => pay_fine(pool, citizen_id, parameters).await,
        "file-dispute-claim" => file_dispute_claim(pool, citizen_id, parameters).await,
        "request-name-change" => request_name_change(pool, citizen_id, parameters).await,
        _ => Err(MojError::InvalidAction(format!(
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

async fn pay_fine(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MojError> {
    let fine_number = parameters
        .get("fineNumber")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| MojError::InvalidAction("fineNumber is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Payment for fine {fine_number} has been received."),
        "fineNumber": fine_number,
    }))
}

async fn file_dispute_claim(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MojError> {
    let claim_type = parameters
        .get("claimType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MojError::InvalidAction("claimType is required".to_owned()))?;

    if !matches!(claim_type, "consumer" | "tenancy" | "debt") {
        return Err(MojError::InvalidAction(format!(
            "claimType must be one of consumer, tenancy, debt (got '{claim_type}')"
        )));
    }

    let description = parameters
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MojError::InvalidAction("description must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Disputes Tribunal claim filed ({claim_type}). We will contact you with a hearing date."),
        "claimType": claim_type,
        "description": description,
    }))
}

async fn request_name_change(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MojError> {
    let new_name = parameters
        .get("newName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MojError::InvalidAction("newName must not be empty".to_owned()))?;

    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MojError::InvalidAction("reason must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Name change request to '{new_name}' submitted for review."),
        "newName": new_name,
        "reason": reason,
    }))
}
