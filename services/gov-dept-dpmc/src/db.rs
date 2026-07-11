use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub dpmc_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct HonoursRow {
    pub id: Uuid,
    pub award_year: i32,
    pub award: String,
    pub status: String,
}

pub async fn fetch_honours(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<HonoursRow>> {
    sqlx::query_as!(
        HonoursRow,
        "SELECT id, award_year, award, status FROM dpmc_honours WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct EngagementsRow {
    pub id: Uuid,
    pub event_name: String,
    pub event_date: chrono::NaiveDate,
    pub location: String,
}

pub async fn fetch_engagements(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<EngagementsRow>> {
    sqlx::query_as!(
        EngagementsRow,
        "SELECT id, event_name, event_date, location FROM dpmc_engagements WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, dpmc_id FROM citizens WHERE did = $1",
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
