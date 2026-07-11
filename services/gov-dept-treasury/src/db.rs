use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub treasury_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BudgetRow {
    pub id: Uuid,
    pub fiscal_year: i32,
    pub portfolio: String,
    pub appropriation: String,
    pub amount: f64,
}

pub async fn fetch_budget(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<BudgetRow>> {
    sqlx::query_as!(
        BudgetRow,
        "SELECT id, fiscal_year, portfolio, appropriation, amount FROM treasury_budget WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct EconomicOutlookRow {
    pub id: Uuid,
    pub forecast_year: i32,
    pub gdp_growth_pct: f64,
    pub inflation_pct: f64,
    pub net_debt_pct: f64,
}

pub async fn fetch_economic_outlook(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<EconomicOutlookRow>> {
    sqlx::query_as!(
        EconomicOutlookRow,
        "SELECT id, forecast_year, gdp_growth_pct, inflation_pct, net_debt_pct FROM treasury_economic_outlook WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, treasury_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn log_action(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: serde_json::Value,
    performed_by: &str,
    ai_level: Option<&str>,
    result_success: bool,
    result_message: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO actions_log (citizen_id, action_type, parameters, performed_by, ai_level, result_success, result_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        citizen_id,
        action_type,
        parameters,
        performed_by,
        ai_level,
        result_success,
        result_message
    )
    .execute(pool)
    .await?;
    Ok(())
}
