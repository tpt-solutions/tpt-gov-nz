use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::DiaError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, DiaError> {
    let result = match action_type {
        "request-passport-renewal" => request_passport_renewal(pool, citizen_id, parameters).await,
        "request-birth-certificate" => request_birth_certificate(pool, citizen_id, parameters).await,
        _ => Err(DiaError::InvalidAction(format!("Unknown action: {action_type}"))),
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

async fn request_passport_renewal(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, DiaError> {
    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("Standard renewal");

    if reason.trim().is_empty() {
        return Err(DiaError::InvalidAction("reason must not be empty".to_owned()));
    }

    db::insert_action_request(pool, citizen_id, "request-passport-renewal", reason).await?;

    Ok(json!({
        "success": true,
        "message": "Passport renewal requested. You will receive an application pack by post.",
    }))
}

async fn request_birth_certificate(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, DiaError> {
    let copies = parameters
        .get("copies")
        .and_then(|v| v.as_i64())
        .unwrap_or(1);

    if copies < 1 || copies > 10 {
        return Err(DiaError::InvalidAction("copies must be between 1 and 10".to_owned()));
    }

    let detail = format!("Birth certificate request — {copies} copy(ies)");
    db::insert_action_request(pool, citizen_id, "request-birth-certificate", &detail).await?;

    Ok(json!({
        "success": true,
        "message": format!("Birth certificate request submitted for {copies} copy(ies)."),
    }))
}
