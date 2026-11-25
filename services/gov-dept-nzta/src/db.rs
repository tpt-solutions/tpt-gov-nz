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
    pub licence_number: String,
    pub full_name: String,
    pub licence_class: String,
    pub expiry_date: chrono::NaiveDate,
    pub conditions: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VehicleRow {
    pub registration: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub fuel_type: String,
    pub registration_expiry: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RucRow {
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

pub async fn fetch_driver_licence(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<DriverLicenceRow>> {
    sqlx::query_as!(
        DriverLicenceRow,
        "SELECT licence_number, full_name, licence_class, expiry_date, conditions FROM driver_licences WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_vehicles(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<VehicleRow>> {
    sqlx::query_as!(
        VehicleRow,
        "SELECT registration, make, model, year, fuel_type, registration_expiry FROM vehicles WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_ruc(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<RucRow>> {
    sqlx::query_as!(
        RucRow,
        "SELECT vehicle_rego, licence_type, expiry_date, units_remaining FROM ruc_records WHERE citizen_id = $1",
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

/// One row for `data_access_log`.
#[derive(Debug)]
pub struct DataAccessEntry<'a> {
    pub citizen_id: Uuid,
    pub requesting_dept_id: &'a str,
    pub scopes: &'a [String],
    pub direct_access: bool,
    pub granted: bool,
}

/// Append a read-audit row. Callers must propagate the error: a request whose access cannot be
/// recorded must not return data.
pub async fn log_data_access(pool: &PgPool, entry: DataAccessEntry<'_>) -> sqlx::Result<()> {
    sqlx::query(
        r#"INSERT INTO data_access_log (citizen_id, requesting_dept_id, scopes, direct_access, outcome)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(entry.citizen_id)
    .bind(entry.requesting_dept_id)
    .bind(entry.scopes)
    .bind(entry.direct_access)
    .bind(if entry.granted { "granted" } else { "denied" })
    .execute(pool)
    .await?;
    Ok(())
}
