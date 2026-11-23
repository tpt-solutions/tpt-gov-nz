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
    pub has_plan: bool,
    pub retirement_age: i32,
    pub last_review: chrono::NaiveDate,
}

pub async fn fetch_retirement_plan(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<RetirementPlanRow>> {
    sqlx::query_as!(
        RetirementPlanRow,
        "SELECT has_plan, retirement_age, last_review FROM retirement_retirement_plan WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct GuidanceRow {
    pub topic: String,
    pub summary: String,
    pub published: chrono::NaiveDate,
}

pub async fn fetch_guidance(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<GuidanceRow>> {
    sqlx::query_as!(
        GuidanceRow,
        "SELECT topic, summary, published FROM retirement_guidance WHERE citizen_id = $1 ORDER BY created_at DESC",
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
