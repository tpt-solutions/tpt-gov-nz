use axum::Json;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MaritimeError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MaritimeError> {
    let result = match action_type {
        "report-incident" => report_incident(pool, citizen_id, parameters).await,
        _ => Err(MaritimeError::InvalidAction(format!(
            "Unknown action: {action_type}"
        ))),
    };

    let (success, message) = match &result {
        Ok(_) => (true, None),
        Err(e) => (false, Some(e.to_string())),
    };

    db::log_action(
        pool,
        db::ActionLogEntry {
            citizen_id,
            action_type,
            parameters: parameters.clone(),
            performed_by,
            ai_level,
            result_success: success,
            result_message: message.as_deref(),
        },
    )
    .await?;

    result.map(Json)
}

async fn report_incident(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MaritimeError> {
    let incident_type = parameters
        .get("incidentType")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MaritimeError::InvalidAction("incidentType must not be empty".into()))?;

    Ok(json!({
        "success": true,
        "message": format!("{}{}", "Incident report received for type", incident_type),
        "incidentType": incident_type,
    }))
}
