use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub corrections_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProbationRow {
    pub id: Uuid,
    pub citizen_id: Uuid,
    pub status: String,
    pub officer_name: String,
    pub next_report_date: chrono::NaiveDate,
    pub location: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CaseRow {
    pub id: Uuid,
    pub citizen_id: Uuid,
    pub case_number: String,
    pub sentence_type: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub summary: String,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, corrections_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_probation(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<ProbationRow>> {
    sqlx::query_as!(
        ProbationRow,
        "SELECT id, citizen_id, status, officer_name, next_report_date, location \
         FROM corrections_probation WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_cases(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<CaseRow>> {
    sqlx::query_as!(
        CaseRow,
        "SELECT id, citizen_id, case_number, sentence_type, start_date, end_date, summary \
         FROM corrections_case WHERE citizen_id = $1 ORDER BY start_date DESC",
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
