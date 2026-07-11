use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::CustomsError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, CustomsError> {
    let result = match action_type {
        "submit-traveller-declaration" => submit_traveller_declaration(parameters).await,
        _ => Err(CustomsError::InvalidAction(format!(
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

async fn submit_traveller_declaration(
    _parameters: &Value,
) -> Result<Value, CustomsError> {
    let country_from = _parameters
        .get("countryFrom")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| CustomsError::InvalidAction("countryFrom is required".to_owned()))?;

    let goods_declared = _parameters
        .get("goodsDeclared")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| CustomsError::InvalidAction("goodsDeclared must not be empty".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Traveller declaration submitted for {country_from}."),
        "countryFrom": country_from,
        "goodsDeclared": goods_declared,
    }))
}
