use sqlx::PgPool;
use uuid::Uuid;

/// Raw DB rows — internal only, never exposed in API responses

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BenefitRow {
    pub benefit_type: String,
    pub weekly_amount: sqlx::types::Decimal,
    pub start_date: Option<chrono::NaiveDate>,
    pub review_date: Option<chrono::NaiveDate>,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PaymentRow {
    pub id: Uuid,
    pub benefit_type: String,
    pub payment_date: chrono::NaiveDate,
    pub amount: sqlx::types::Decimal,
    pub method: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CaseNoteRow {
    pub id: Uuid,
    pub note_date: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub note: String,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, client_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_benefits(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<BenefitRow>> {
    sqlx::query_as!(
        BenefitRow,
        r#"SELECT benefit_type, weekly_amount, start_date, review_date, status
           FROM benefits
           WHERE citizen_id = $1
           ORDER BY weekly_amount DESC"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_payments(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<PaymentRow>> {
    sqlx::query_as!(
        PaymentRow,
        r#"SELECT id, benefit_type, payment_date, amount, method
           FROM payments
           WHERE citizen_id = $1
           ORDER BY payment_date DESC
           LIMIT 24"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_case_notes(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<CaseNoteRow>> {
    sqlx::query_as!(
        CaseNoteRow,
        r#"SELECT id, note_date, author, note
           FROM case_notes
           WHERE citizen_id = $1
           ORDER BY note_date DESC
           LIMIT 50"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_case_note(
    pool: &PgPool,
    citizen_id: Uuid,
    author: &str,
    note: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO case_notes (citizen_id, author, note) VALUES ($1, $2, $3)",
        citizen_id,
        author,
        note
    )
    .execute(pool)
    .await?;
    Ok(())
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
