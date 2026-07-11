use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub regulation_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RegulatoryReviewsRow {
    pub id: Uuid,
    pub topic: String,
    pub agency: String,
    pub status: String,
    pub review_year: i32,
}

pub async fn fetch_regulatory_reviews(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<RegulatoryReviewsRow>> {
    sqlx::query_as!(
        RegulatoryReviewsRow,
        "SELECT id, topic, agency, status, review_year FROM regulation_regulatory_reviews WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProposalsRow {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

pub async fn fetch_proposals(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ProposalsRow>> {
    sqlx::query_as!(
        ProposalsRow,
        "SELECT id, title, status FROM regulation_proposals WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, regulation_id FROM citizens WHERE did = $1",
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
