use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::AccError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, AccError> {
    let result = match action_type {
        "lodge-claim" => lodge_claim(pool, citizen_id, parameters).await,
        "request-rehabilitation-review" => {
            request_rehabilitation_review(pool, citizen_id, parameters).await
        }
        _ => Err(AccError::InvalidAction(format!(
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

async fn lodge_claim(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, AccError> {
    let claim_type = parameters
        .get("claimType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AccError::InvalidAction("claimType is required".to_owned()))?;

    if !matches!(claim_type, "work" | "non-work" | "treatment") {
        return Err(AccError::InvalidAction(format!(
            "claimType must be one of work, non-work, treatment (got '{claim_type}')"
        )));
    }

    let injury_date = parameters
        .get("injuryDate")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AccError::InvalidAction("injuryDate is required".to_owned()))?;

    let description = parameters
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AccError::InvalidAction("description must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("ACC claim lodged ({claim_type}). Our team will assess your claim for the injury on {injury_date}."),
        "claimType": claim_type,
        "injuryDate": injury_date,
        "description": description,
    }))
}

async fn request_rehabilitation_review(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, AccError> {
    let claim_number = parameters
        .get("claimNumber")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    let detail = match &claim_number {
        Some(c) => format!("Rehabilitation review requested for claim {c}."),
        None => "Rehabilitation review requested.".to_owned(),
    };

    Ok(json!({
        "success": true,
        "message": detail,
        "claimNumber": claim_number,
    }))
}
