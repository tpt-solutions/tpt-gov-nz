use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MpiError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MpiError> {
    let result = match action_type {
        "apply-export-certificate" => apply_export_certificate(pool, citizen_id, parameters).await,
        _ => Err(MpiError::InvalidAction(format!(
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

async fn apply_export_certificate(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MpiError> {
    let product = parameters
        .get("product")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MpiError::InvalidAction("product is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Export certificate application for '{product}' received. MPI will review and issue the certificate if requirements are met."),
        "product": product,
    }))
}
