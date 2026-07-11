use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::MsdError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, MsdError> {
    let result = match action_type {
        "apply-student-allowance" => apply_student_allowance(pool, citizen_id, parameters).await,
        "update-loan-repayment-plan" => update_loan_repayment_plan(pool, citizen_id, parameters).await,
        _ => Err(MsdError::InvalidAction(format!(
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

async fn apply_student_allowance(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MsdError> {
    let course_of_study = parameters
        .get("courseOfStudy")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MsdError::InvalidAction("courseOfStudy is required".to_owned()))?;

    let provider = parameters
        .get("provider")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MsdError::InvalidAction("provider is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Student allowance application for {course_of_study} at {provider} submitted. StudyLink will assess your application and be in touch."),
        "courseOfStudy": course_of_study,
        "provider": provider,
    }))
}

async fn update_loan_repayment_plan(
    _pool: &PgPool,
    _citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, MsdError> {
    let plan = parameters
        .get("plan")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| MsdError::InvalidAction("plan is required".to_owned()))?;

    Ok(json!({
        "success": true,
        "message": format!("Student loan repayment plan updated to '{plan}'. Inland Revenue will apply the new plan to your loan."),
        "plan": plan,
    }))
}
