use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::NztaError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, NztaError> {
    let result = match action_type {
        "renew-vehicle-registration" => renew_vehicle_registration(citizen_id, parameters).await,
        "request-licence-replacement" => request_licence_replacement(citizen_id, parameters).await,
        _ => Err(NztaError::InvalidAction(format!("Unknown action: {action_type}"))),
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

async fn renew_vehicle_registration(
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, NztaError> {
    let registration = parameters
        .get("registration")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned();

    if registration.trim().is_empty() {
        return Err(NztaError::InvalidAction("registration must not be empty".to_owned()));
    }

    let months = parameters
        .get("months")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    if !(1..=24).contains(&months) {
        return Err(NztaError::InvalidAction("months must be between 1 and 24".to_owned()));
    }

    Ok(json!({
        "success": true,
        "message": format!("Vehicle registration {registration} renewed for {months} month(s)."),
    }))
}

async fn request_licence_replacement(
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, NztaError> {
    let reason = parameters
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if reason.trim().is_empty() {
        return Err(NztaError::InvalidAction("reason must not be empty".to_owned()));
    }

    Ok(json!({
        "success": true,
        "message": "Driver licence replacement requested. Your new licence card will be posted to your address.",
    }))
}
