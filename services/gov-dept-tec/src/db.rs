use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub tec_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FundingRow {
    pub id: Uuid,
    pub provider: String,
    pub amount: f64,
    pub year: i32,
}

pub async fn fetch_funding(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<FundingRow>> {
    sqlx::query_as!(
        FundingRow,
        "SELECT id, provider, amount, year FROM tec_funding WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct CoursesRow {
    pub id: Uuid,
    pub course_name: String,
    pub provider: String,
    pub status: String,
}

pub async fn fetch_courses(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<CoursesRow>> {
    sqlx::query_as!(
        CoursesRow,
        "SELECT id, course_name, provider, status FROM tec_courses WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, tec_id FROM citizens WHERE did = $1",
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
