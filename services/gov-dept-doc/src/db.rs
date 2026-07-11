use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub doc_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PermitRow {
    pub id: Uuid,
    pub permit_number: String,
    pub activity: String,
    pub location: String,
    pub status: String,
    pub expires_date: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ConcessionRow {
    pub id: Uuid,
    pub concession_id: String,
    pub r#type: String,
    pub holder: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, doc_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_permits(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<PermitRow>> {
    sqlx::query_as!(
        PermitRow,
        "SELECT id, permit_number, activity, location, status, expires_date \
         FROM doc_permits WHERE citizen_id = $1 ORDER BY expires_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_concessions(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<ConcessionRow>> {
    sqlx::query_as!(
        ConcessionRow,
        "SELECT id, concession_id, type, holder, start_date, end_date \
         FROM doc_concessions WHERE citizen_id = $1 ORDER BY start_date DESC",
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
