use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::WinzError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, WinzError> {
    let result = match action_type {
        "request-appointment" => request_appointment(pool, citizen_id, parameters).await,
        "submit-benefit-review" => submit_benefit_review(pool, citizen_id, parameters).await,
        _ => Err(WinzError::InvalidAction(format!("Unknown action: {action_type}"))),
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

async fn request_appointment(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, WinzError> {
    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .ok_or_else(|| WinzError::InvalidAction("reason required".to_owned()))?;

    if reason.trim().is_empty() {
        return Err(WinzError::InvalidAction("reason must not be empty".to_owned()));
    }

    let channel = parameters
        .get("channel")
        .and_then(|v| v.as_str())
        .unwrap_or("phone");

    let note = format!("Appointment requested via {channel}. Reason: {reason}");
    db::insert_case_note(pool, citizen_id, "citizen", &note).await?;

    Ok(json!({
        "success": true,
        "message": "Appointment request received. A case worker will be in touch.",
        "channel": channel,
    }))
}

async fn submit_benefit_review(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, WinzError> {
    let notes = parameters
        .get("notes")
        .and_then(|v| v.as_str())
        .ok_or_else(|| WinzError::InvalidAction("notes required".to_owned()))?;

    let note = format!("Benefit review submitted. Citizen notes: {notes}");
    db::insert_case_note(pool, citizen_id, "citizen", &note).await?;

    Ok(json!({
        "success": true,
        "message": "Benefit review submitted. Your case manager will review your circumstances.",
    }))
}
