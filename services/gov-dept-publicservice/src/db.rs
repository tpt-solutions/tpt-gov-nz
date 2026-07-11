use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub psc_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct WorkforceRow {
    pub id: Uuid,
    pub report_year: i32,
    pub agency: String,
    pub headcount: i32,
}

pub async fn fetch_workforce(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<WorkforceRow>> {
    sqlx::query_as!(
        WorkforceRow,
        "SELECT id, report_year, agency, headcount FROM publicservice_workforce WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct AgencyRatingsRow {
    pub id: Uuid,
    pub agency: String,
    pub rating: String,
    pub rating_year: i32,
}

pub async fn fetch_agency_ratings(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<AgencyRatingsRow>> {
    sqlx::query_as!(
        AgencyRatingsRow,
        "SELECT id, agency, rating, rating_year FROM publicservice_agency_ratings WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, psc_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
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
