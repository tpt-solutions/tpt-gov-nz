use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub eqc_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClaimsRow {
    pub id: Uuid,
    pub reference: String,
    pub property: String,
    pub status: String,
    pub lodged_date: chrono::NaiveDate,
}

pub async fn fetch_claims(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ClaimsRow>> {
    sqlx::query_as!(
        ClaimsRow,
        "SELECT id, reference, property, status, lodged_date FROM eqc_claims WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct CoverRow {
    pub id: Uuid,
    pub property: String,
    pub sum_insured: f64,
    pub valid_to: chrono::NaiveDate,
}

pub async fn fetch_cover(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<CoverRow>> {
    sqlx::query_as!(
        CoverRow,
        "SELECT id, property, sum_insured, valid_to FROM eqc_cover WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, eqc_id FROM citizens WHERE did = $1",
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
