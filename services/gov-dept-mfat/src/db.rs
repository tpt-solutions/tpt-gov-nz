use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub mfat_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OverseasMissionsRow {
    pub id: Uuid,
    pub country: String,
    pub status: String,
}

pub async fn fetch_overseas_missions(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<OverseasMissionsRow>> {
    sqlx::query_as!(
        OverseasMissionsRow,
        "SELECT id, country, status FROM mfat_overseas_missions WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct TravelAdvisoriesRow {
    pub id: Uuid,
    pub country: String,
    pub level: String,
    pub updated: chrono::NaiveDate,
}

pub async fn fetch_travel_advisories(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<TravelAdvisoriesRow>> {
    sqlx::query_as!(
        TravelAdvisoriesRow,
        "SELECT id, country, level, updated FROM mfat_travel_advisories WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, mfat_id FROM citizens WHERE did = $1",
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
