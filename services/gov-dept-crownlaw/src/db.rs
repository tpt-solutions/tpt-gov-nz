use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub crownlaw_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LegalOpinionsRow {
    pub id: Uuid,
    pub reference: String,
    pub topic: String,
    pub issued_date: chrono::NaiveDate,
    pub status: String,
}

pub async fn fetch_legal_opinions(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<LegalOpinionsRow>> {
    sqlx::query_as!(
        LegalOpinionsRow,
        "SELECT id, reference, topic, issued_date, status FROM crownlaw_legal_opinions WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct LitigationRow {
    pub id: Uuid,
    pub case_name: String,
    pub crown_role: String,
    pub status: String,
}

pub async fn fetch_litigation(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<LitigationRow>> {
    sqlx::query_as!(
        LitigationRow,
        "SELECT id, case_name, crown_role, status FROM crownlaw_litigation WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, crownlaw_id FROM citizens WHERE did = $1",
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
