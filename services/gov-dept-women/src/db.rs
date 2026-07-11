use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub women_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProgrammesRow {
    pub id: Uuid,
    pub programme_name: String,
    pub status: String,
    pub year: i32,
}

pub async fn fetch_programmes(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ProgrammesRow>> {
    sqlx::query_as!(
        ProgrammesRow,
        "SELECT id, programme_name, status, year FROM women_programmes WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct InsightsRow {
    pub id: Uuid,
    pub topic: String,
    pub summary: String,
    pub published: chrono::NaiveDate,
}

pub async fn fetch_insights(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<InsightsRow>> {
    sqlx::query_as!(
        InsightsRow,
        "SELECT id, topic, summary, published FROM women_insights WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, women_id FROM citizens WHERE did = $1",
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
