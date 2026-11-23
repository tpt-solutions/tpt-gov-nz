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
    pub permit_number: String,
    pub activity: String,
    pub location: String,
    pub status: String,
    pub expires_date: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ConcessionRow {
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

pub async fn fetch_permits(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<PermitRow>> {
    sqlx::query_as!(
        PermitRow,
        "SELECT permit_number, activity, location, status, expires_date FROM doc_permits WHERE citizen_id = $1 ORDER BY expires_date DESC",
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
        "SELECT concession_id, type, holder, start_date, end_date FROM doc_concessions WHERE citizen_id = $1 ORDER BY start_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

/// One row for `actions_log`.
#[derive(Debug)]
pub struct ActionLogEntry<'a> {
    pub citizen_id: Uuid,
    pub action_type: &'a str,
    pub parameters: serde_json::Value,
    pub performed_by: &'a str,
    pub ai_level: Option<&'a str>,
    pub result_success: bool,
    pub result_message: Option<&'a str>,
}

pub async fn log_action(pool: &PgPool, entry: ActionLogEntry<'_>) -> sqlx::Result<()> {
    let ActionLogEntry {
        citizen_id,
        action_type,
        parameters,
        performed_by,
        ai_level,
        result_success,
        result_message,
    } = entry;
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
