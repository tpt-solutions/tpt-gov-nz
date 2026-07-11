use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub driver_licence_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DriverLicenceRow {
    pub id: Uuid,
    pub licence_number: String,
    pub full_name: String,
    pub licence_class: String,
    pub expiry_date: chrono::NaiveDate,
    pub conditions: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VehicleRow {
    pub id: Uuid,
    pub registration: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub fuel_type: String,
    pub registration_expiry: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RucRow {
    pub id: Uuid,
    pub vehicle_rego: String,
    pub licence_type: String,
    pub expiry_date: chrono::NaiveDate,
    pub units_remaining: i32,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, driver_licence_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_driver_licence(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<DriverLicenceRow>> {
    sqlx::query_as!(
        DriverLicenceRow,
        "SELECT id, licence_number, full_name, licence_class, expiry_date, conditions FROM driver_licences WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_vehicles(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<VehicleRow>> {
    sqlx::query_as!(
        VehicleRow,
        "SELECT id, registration, make, model, year, fuel_type, registration_expiry FROM vehicles WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_ruc(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<RucRow>> {
    sqlx::query_as!(
        RucRow,
        "SELECT id, vehicle_rego, licence_type, expiry_date, units_remaining FROM ruc_records WHERE citizen_id = $1",
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
