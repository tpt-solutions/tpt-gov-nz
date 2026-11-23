use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApplicationRow {

    pub application_number: String,
    pub application_type: String,
    pub status: String,
    pub priority_band: Option<String>,
    pub bedrooms_needed: Option<i32>,
    pub submitted_date: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TenancyRow {

    pub tenancy_id: String,
    pub property_address: String,
    pub weekly_rent: f64,
    pub income_related_rent: bool,
    pub start_date: chrono::NaiveDate,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MaintenanceRequestRow {

    pub request_number: String,
    pub category: String,
    pub status: String,
    pub description: String,
    pub requested_date: chrono::NaiveDate,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, client_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_applications(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<ApplicationRow>> {
    sqlx::query_as!(
        ApplicationRow,
        "SELECT application_number, application_type, status, priority_band, bedrooms_needed, submitted_date FROM hud_applications WHERE citizen_id = $1 ORDER BY submitted_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_tenancies(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<TenancyRow>> {
    sqlx::query_as!(
        TenancyRow,
        "SELECT tenancy_id, property_address, weekly_rent, income_related_rent, start_date, status FROM hud_tenancies WHERE citizen_id = $1 ORDER BY start_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_maintenance_requests(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<MaintenanceRequestRow>> {
    sqlx::query_as!(
        MaintenanceRequestRow,
        "SELECT request_number, category, status, description, requested_date FROM hud_maintenance_requests WHERE citizen_id = $1 ORDER BY requested_date DESC",
        citizen_id
    )
    .fetch_all(pool)
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
