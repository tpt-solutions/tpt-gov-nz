use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub oranga_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CarePlacementsRow {
    pub id: Uuid,
    pub placement_type: String,
    pub start_date: chrono::NaiveDate,
    pub region: String,
}

pub async fn fetch_care_placements(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<CarePlacementsRow>> {
    sqlx::query_as!(
        CarePlacementsRow,
        "SELECT id, placement_type, start_date, region FROM oranga_care_placements WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct SupportServicesRow {
    pub id: Uuid,
    pub service: String,
    pub status: String,
    pub next_review: chrono::NaiveDate,
}

pub async fn fetch_support_services(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<SupportServicesRow>> {
    sqlx::query_as!(
        SupportServicesRow,
        "SELECT id, service, status, next_review FROM oranga_support_services WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, oranga_id FROM citizens WHERE did = $1",
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
