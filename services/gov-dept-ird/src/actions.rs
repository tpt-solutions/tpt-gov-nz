use axum::Json;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db, error::IrdError};

pub async fn execute(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: &Value,
    performed_by: &str,
    ai_level: Option<&str>,
) -> Result<Json<Value>, IrdError> {
    let result = match action_type {
        "update-kiwisaver-rate" => update_kiwisaver_rate(pool, citizen_id, parameters).await,
        "file-gst-return" => file_gst_return(pool, citizen_id, parameters).await,
        "request-tax-summary" => Ok(json!({ "success": true, "message": "Tax summary queued" })),
        _ => Err(IrdError::InvalidAction(format!("Unknown action: {action_type}"))),
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

async fn update_kiwisaver_rate(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, IrdError> {
    let new_rate = parameters
        .get("newRate")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| IrdError::InvalidAction("newRate must be a number".to_owned()))?;

    let valid_rates = [3.0, 4.0, 6.0, 8.0, 10.0];
    if !valid_rates.contains(&new_rate) {
        return Err(IrdError::InvalidAction(
            format!("newRate must be one of: {valid_rates:?}")
        ));
    }

    db::update_kiwisaver_rate(pool, citizen_id, new_rate).await?;

    Ok(json!({
        "success": true,
        "message": format!("KiwiSaver contribution rate updated to {new_rate}%"),
        "newRate": new_rate,
    }))
}

async fn file_gst_return(
    pool: &PgPool,
    citizen_id: Uuid,
    parameters: &Value,
) -> Result<Value, IrdError> {
    let period_id_str = parameters
        .get("periodId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| IrdError::InvalidAction("periodId required".to_owned()))?;

    let period_id = Uuid::parse_str(period_id_str)
        .map_err(|_| IrdError::InvalidAction("periodId must be a valid UUID".to_owned()))?;

    let sales_income = parameters.get("salesIncome").and_then(|v| v.as_f64())
        .ok_or_else(|| IrdError::InvalidAction("salesIncome required".to_owned()))?;
    let gst_on_sales = parameters.get("gstOnSales").and_then(|v| v.as_f64())
        .ok_or_else(|| IrdError::InvalidAction("gstOnSales required".to_owned()))?;
    let gst_on_purchases = parameters.get("gstOnPurchases").and_then(|v| v.as_f64())
        .ok_or_else(|| IrdError::InvalidAction("gstOnPurchases required".to_owned()))?;

    db::file_gst_return(pool, citizen_id, period_id, sales_income, gst_on_sales, gst_on_purchases).await?;

    Ok(json!({
        "success": true,
        "message": "GST return filed successfully",
        "refundOrPayment": gst_on_purchases - gst_on_sales,
    }))
}
