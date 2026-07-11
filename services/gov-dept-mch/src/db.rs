use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub mch_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct HeritageSitesRow {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub region: String,
}

pub async fn fetch_heritage_sites(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<HeritageSitesRow>> {
    sqlx::query_as!(
        HeritageSitesRow,
        "SELECT id, name, status, region FROM mch_heritage_sites WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct GrantsRow {
    pub id: Uuid,
    pub grant_name: String,
    pub amount: f64,
    pub status: String,
}

pub async fn fetch_grants(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<GrantsRow>> {
    sqlx::query_as!(
        GrantsRow,
        "SELECT id, grant_name, amount, status FROM mch_grants WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, mch_id FROM citizens WHERE did = $1",
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
