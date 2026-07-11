use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FineRow {
    pub id: Uuid,
    pub fine_number: String,
    pub fine_type: String,
    pub status: String,
    pub amount: f64,
    pub offense_date: chrono::NaiveDate,
    pub due_date: chrono::NaiveDate,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DisputeRow {
    pub id: Uuid,
    pub dispute_number: String,
    pub claim_type: String,
    pub status: String,
    pub amount_claimed: Option<f64>,
    pub hearing_date: Option<chrono::NaiveDate>,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CourtRecordRow {
    pub id: Uuid,
    pub case_number: String,
    pub case_type: String,
    pub status: String,
    pub next_hearing_date: Option<chrono::NaiveDate>,
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

pub async fn fetch_fines(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<FineRow>> {
    sqlx::query_as!(
        FineRow,
        "SELECT id, fine_number, fine_type, status, amount, offense_date, due_date, description \
         FROM moj_fines WHERE citizen_id = $1 ORDER BY due_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_disputes(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<DisputeRow>> {
    sqlx::query_as!(
        DisputeRow,
        "SELECT id, dispute_number, claim_type, status, amount_claimed, hearing_date, description \
         FROM moj_disputes WHERE citizen_id = $1 ORDER BY hearing_date NULLS LAST",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_court_records(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<CourtRecordRow>> {
    sqlx::query_as!(
        CourtRecordRow,
        "SELECT id, case_number, case_type, status, next_hearing_date, description \
         FROM moj_court_records WHERE citizen_id = $1 ORDER BY next_hearing_date NULLS LAST",
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
