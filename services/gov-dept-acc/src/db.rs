use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClaimRow {
    pub id: Uuid,
    pub claim_number: String,
    pub claim_type: String,
    pub status: String,
    pub injury_date: chrono::NaiveDate,
    pub description: String,
    pub weekly_compensation: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EntitlementRow {
    pub id: Uuid,
    pub has_entitlement: bool,
    pub r#type: Option<String>,
    pub weekly_amount: Option<f64>,
    pub remaining_weeks: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RehabilitationRow {
    pub id: Uuid,
    pub plan_id: String,
    pub description: String,
    pub status: String,
    pub provider: Option<String>,
    pub next_review: Option<chrono::NaiveDate>,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, client_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_claims(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ClaimRow>> {
    sqlx::query_as!(
        ClaimRow,
        "SELECT id, claim_number, claim_type, status, injury_date, description, weekly_compensation \
         FROM acc_claims WHERE citizen_id = $1 ORDER BY injury_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_entitlement(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<EntitlementRow>> {
    sqlx::query_as!(
        EntitlementRow,
        "SELECT id, has_entitlement, type, weekly_amount, remaining_weeks \
         FROM acc_entitlements WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_rehabilitation(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<RehabilitationRow>> {
    sqlx::query_as!(
        RehabilitationRow,
        "SELECT id, plan_id, description, status, provider, next_review \
         FROM acc_rehabilitation WHERE citizen_id = $1 ORDER BY next_review NULLS LAST",
        citizen_id
    )
    .fetch_all(pool)
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
