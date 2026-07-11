use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub tpk_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProgrammeRow {
    pub id: Uuid,
    pub programme_name: String,
    pub status: String,
    pub region: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FundingRow {
    pub id: Uuid,
    pub grant_id: String,
    pub amount: i32,
    pub purpose: String,
    pub status: String,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, tpk_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_programmes(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<ProgrammeRow>> {
    sqlx::query_as!(
        ProgrammeRow,
        "SELECT id, programme_name, status, region \
         FROM tpk_programmes WHERE citizen_id = $1 ORDER BY programme_name ASC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_funding(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<FundingRow>> {
    sqlx::query_as!(
        FundingRow,
        "SELECT id, grant_id, amount, purpose, status \
         FROM tpk_funding WHERE citizen_id = $1 ORDER BY grant_id ASC",
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
