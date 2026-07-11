use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub person_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BusinessRow {
    pub id: Uuid,
    pub nzbn: String,
    pub entity_name: String,
    pub entity_type: String,
    pub status: String,
    pub registered_date: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DirectorshipRow {
    pub id: Uuid,
    pub nzbn: String,
    pub entity_name: String,
    pub role: String,
    pub appointed_date: chrono::NaiveDate,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, person_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_business_registrations(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<BusinessRow>> {
    sqlx::query_as!(
        BusinessRow,
        "SELECT id, nzbn, entity_name, entity_type, status, registered_date \
         FROM mbie_business_registrations WHERE citizen_id = $1 ORDER BY registered_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_directorships(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<DirectorshipRow>> {
    sqlx::query_as!(
        DirectorshipRow,
        "SELECT id, nzbn, entity_name, role, appointed_date \
         FROM mbie_directorships WHERE citizen_id = $1 ORDER BY appointed_date DESC",
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
