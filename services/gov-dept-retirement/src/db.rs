use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub retirement_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RetirementPlanRow {
    pub id: Uuid,
    pub has_plan: bool,
    pub retirement_age: i32,
    pub last_review: chrono::NaiveDate,
}

pub async fn fetch_retirement_plan(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<RetirementPlanRow>> {
    sqlx::query_as!(
        RetirementPlanRow,
        "SELECT id, has_plan, retirement_age, last_review FROM retirement_retirement_plan WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct GuidanceRow {
    pub id: Uuid,
    pub topic: String,
    pub summary: String,
    pub published: chrono::NaiveDate,
}

pub async fn fetch_guidance(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<GuidanceRow>> {
    sqlx::query_as!(
        GuidanceRow,
        "SELECT id, topic, summary, published FROM retirement_guidance WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, retirement_id FROM citizens WHERE did = $1",
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
