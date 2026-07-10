use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MohError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MohError> {
    let result = match action_type {
        "request-repeat-prescription" => request_repeat_prescription(pool, citizen_id, parameters).await,
        "book-appointment" => book_appointment(pool, citizen_id, parameters).await,
        _ => Err(MohError::InvalidAction(format!("Unknown action: {action_type}"))),
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

async fn request_repeat_prescription(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MohError> {
    let prescription_id = parameters
        .get("prescriptionId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MohError::InvalidAction("prescriptionId required".to_owned()))?;

    let pid = uuid::Uuid::parse_str(prescription_id)
        .map_err(|_| MohError::InvalidAction("invalid prescriptionId".to_owned()))?;

    let remaining = db::decrement_prescription_repeat(pool, pid, citizen_id).await?;

    match remaining {
        Some(n) => Ok(json!({
            "success": true,
            "message": "Repeat prescription requested. Your pharmacy will be notified.",
            "repeatsRemaining": n,
        })),
        None => Err(MohError::ActionFailed(
            "No repeats remaining, or prescription not found".to_owned(),
        )),
    }
}

async fn book_appointment(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MohError> {
    let provider = parameters
        .get("provider")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| MohError::InvalidAction("provider required".to_owned()))?;

    let appt_type = parameters
        .get("type")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| MohError::InvalidAction("type required".to_owned()))?;

    let date_str = parameters
        .get("date")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MohError::InvalidAction("date required".to_owned()))?;

    let appt_date = chrono::DateTime::<chrono::Utc>::from(
        chrono::DateTime::parse_from_rfc3339(date_str)
            .map_err(|e| MohError::InvalidAction(format!("invalid date: {e}")))?,
    );

    db::insert_appointment(pool, citizen_id, provider, appt_date, appt_type).await?;

    Ok(json!({
        "success": true,
        "message": format!("Appointment with {provider} booked for {date_str}."),
    }))
}
