use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub customer_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TitleRow {
    pub id: Uuid,
    pub title_number: String,
    pub property_address: String,
    pub land_area_sqm: f64,
    pub estate_type: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct OwnershipRow {
    pub id: Uuid,
    pub title_number: String,
    pub ownership_share: String,
    pub registered_owners: serde_json::Value,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, customer_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_titles(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<TitleRow>> {
    sqlx::query_as!(
        TitleRow,
        "SELECT id, title_number, property_address, land_area_sqm, estate_type \
         FROM linz_titles WHERE citizen_id = $1 ORDER BY title_number",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_ownership(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<OwnershipRow>> {
    sqlx::query_as!(
        OwnershipRow,
        "SELECT id, title_number, ownership_share, registered_owners \
         FROM linz_ownership WHERE citizen_id = $1 ORDER BY title_number",
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
