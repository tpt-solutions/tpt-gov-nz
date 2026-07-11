use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct InfringementRow {
    pub id: Uuid,
    pub ticket_number: String,
    pub offense_type: String,
    pub status: String,
    pub amount: f64,
    pub issue_date: chrono::NaiveDate,
    pub location: Option<String>,
    pub demerit_points: Option<i32>,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReportRow {
    pub id: Uuid,
    pub report_number: String,
    pub report_type: String,
    pub status: String,
    pub filed_date: chrono::NaiveDate,
    pub description: String,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, client_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_infringements(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<InfringementRow>> {
    sqlx::query_as!(
        InfringementRow,
        "SELECT id, ticket_number, offense_type, status, amount, issue_date, location, demerit_points, description \
         FROM police_infringements WHERE citizen_id = $1 ORDER BY issue_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_reports(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ReportRow>> {
    sqlx::query_as!(
        ReportRow,
        "SELECT id, report_number, report_type, status, filed_date, description \
         FROM police_reports WHERE citizen_id = $1 ORDER BY filed_date DESC",
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
