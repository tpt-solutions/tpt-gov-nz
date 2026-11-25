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
    pub status: String,
    pub officer_name: String,
    pub next_report_date: Option<chrono::NaiveDate>,
    pub location: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CaseRow {
    pub case_number: String,
    pub sentence_type: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub summary: Option<String>,
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
        "SELECT status, officer_name, next_report_date, location FROM corrections_probation WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_cases(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<CaseRow>> {
    sqlx::query_as!(
        CaseRow,
        "SELECT case_number, sentence_type, start_date, end_date, summary FROM corrections_case WHERE citizen_id = $1 ORDER BY start_date DESC",
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

/// One row for `data_access_log`.
#[derive(Debug)]
pub struct DataAccessEntry<'a> {
    pub citizen_id: Uuid,
    pub requesting_dept_id: &'a str,
    pub scopes: &'a [String],
    pub direct_access: bool,
    pub granted: bool,
}

/// Append a read-audit row. Callers must propagate the error: a request whose access cannot be
/// recorded must not return data.
pub async fn log_data_access(pool: &PgPool, entry: DataAccessEntry<'_>) -> sqlx::Result<()> {
    sqlx::query(
        r#"INSERT INTO data_access_log (citizen_id, requesting_dept_id, scopes, direct_access, outcome)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(entry.citizen_id)
    .bind(entry.requesting_dept_id)
    .bind(entry.scopes)
    .bind(entry.direct_access)
    .bind(if entry.granted { "granted" } else { "denied" })
    .execute(pool)
    .await?;
    Ok(())
}
