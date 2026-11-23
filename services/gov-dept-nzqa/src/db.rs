use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub nsn: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct QualificationRow {

    pub qualification_id: String,
    pub title: String,
    pub level: i32,
    pub awarded_date: chrono::NaiveDate,
    pub provider: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TranscriptRow {

    pub record_summary: Option<String>,
    pub total_credits: Option<i32>,
    pub credit_summary: Option<String>,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, nsn FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_qualifications(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<QualificationRow>> {
    sqlx::query_as!(
        QualificationRow,
        "SELECT qualification_id, title, level, awarded_date, provider FROM nzqa_qualifications WHERE citizen_id = $1 ORDER BY awarded_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_transcript(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<TranscriptRow>> {
    sqlx::query_as!(
        TranscriptRow,
        "SELECT record_summary, total_credits, credit_summary FROM nzqa_transcripts WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
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
