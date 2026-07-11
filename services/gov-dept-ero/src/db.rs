use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub ero_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReviewsRow {
    pub id: Uuid,
    pub school: String,
    pub rating: String,
    pub review_date: chrono::NaiveDate,
    pub next_review: chrono::NaiveDate,
}

pub async fn fetch_reviews(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ReviewsRow>> {
    sqlx::query_as!(
        ReviewsRow,
        "SELECT id, school, rating, review_date, next_review FROM ero_reviews WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReportsRow {
    pub id: Uuid,
    pub title: String,
    pub published: chrono::NaiveDate,
}

pub async fn fetch_reports(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ReportsRow>> {
    sqlx::query_as!(
        ReportsRow,
        "SELECT id, title, published FROM ero_reports WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, ero_id FROM citizens WHERE did = $1",
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
