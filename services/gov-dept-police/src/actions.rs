use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::PoliceError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, PoliceError> {
    let result = match action_type {
        "pay-infringement" => pay_infringement(pool, citizen_id, parameters).await,
        "dispute-infringement" => dispute_infringement(pool, citizen_id, parameters).await,
        "file-report" => file_report(pool, citizen_id, parameters).await,
        _ => Err(PoliceError::InvalidAction(format!(
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

async fn pay_infringement(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, PoliceError> {
    let ticket_number = parameters
        .get("ticketNumber")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| PoliceError::InvalidAction("ticketNumber is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Payment for infringement {ticket_number} has been received."),
        "ticketNumber": ticket_number,
    }))
}

async fn dispute_infringement(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, PoliceError> {
    let ticket_number = parameters
        .get("ticketNumber")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| PoliceError::InvalidAction("ticketNumber is required".to_owned()))?;

    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PoliceError::InvalidAction("reason must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Dispute for infringement {ticket_number} has been lodged."),
        "ticketNumber": ticket_number,
        "reason": reason,
    }))
}

async fn file_report(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, PoliceError> {
    let report_type = parameters
        .get("reportType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| PoliceError::InvalidAction("reportType is required".to_owned()))?;

    if !matches!(report_type, "theft" | "incident" | "lost-property") {
        return Err(PoliceError::InvalidAction(format!(
            "reportType must be one of theft, incident, lost-property (got '{report_type}')"
        )));
    }

    let description = parameters
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PoliceError::InvalidAction("description must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Report filed ({report_type}). A reference number will be sent to you."),
        "reportType": report_type,
        "description": description,
    }))
}
