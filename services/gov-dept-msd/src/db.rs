use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudyLinkRow {
    pub id: Uuid,
    pub citizen_id: Uuid,
    pub has_student_loan: bool,
    pub loan_balance: Option<sqlx::types::Decimal>,
    pub repayment_plan: Option<String>,
    pub has_allowance: bool,
    pub allowance_type: Option<String>,
    pub next_payment_date: Option<chrono::NaiveDate>,
    pub weekly_amount: Option<sqlx::types::Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CaseEventRow {
    pub id: Uuid,
    pub citizen_id: Uuid,
    pub event_id: String,
    pub event_date: chrono::NaiveDate,
    pub service_line: String,
    pub summary: String,
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

pub async fn fetch_studylink(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<StudyLinkRow>> {
    sqlx::query_as!(
        StudyLinkRow,
        "SELECT id, citizen_id, has_student_loan, loan_balance, repayment_plan, has_allowance, allowance_type, next_payment_date, weekly_amount \
         FROM msd_studylink WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_case_history(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<CaseEventRow>> {
    sqlx::query_as!(
        CaseEventRow,
        "SELECT id, citizen_id, event_id, event_date, service_line, summary \
         FROM msd_case_history WHERE citizen_id = $1 ORDER BY event_date DESC",
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
