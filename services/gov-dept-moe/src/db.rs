use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub moe_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EnrolmentRow {
    pub school: String,
    pub year_level: i32,
    pub status: String,
}

pub async fn fetch_enrolment(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<EnrolmentRow>> {
    sqlx::query_as!(
        EnrolmentRow,
        "SELECT school, year_level, status FROM moe_enrolment WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudentSupportRow {
    pub service: String,
    pub status: String,
    pub next_review: chrono::NaiveDate,
}

pub async fn fetch_student_support(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<StudentSupportRow>> {
    sqlx::query_as!(
        StudentSupportRow,
        "SELECT service, status, next_review FROM moe_student_support WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, moe_id FROM citizens WHERE did = $1",
        did
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
