use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub caa_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LicencesRow {
    pub licence_no: String,
    pub category: String,
    pub status: String,
    pub expires: Option<chrono::NaiveDate>,
}

pub async fn fetch_licences(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<LicencesRow>> {
    sqlx::query_as!(
        LicencesRow,
        "SELECT licence_no, category, status, expires FROM caa_licences WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct AircraftRow {
    pub registration: String,
    pub aircraft_type: String,
    pub status: String,
}

pub async fn fetch_aircraft(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<AircraftRow>> {
    sqlx::query_as!(
        AircraftRow,
        "SELECT registration, aircraft_type, status FROM caa_aircraft WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, caa_id FROM citizens WHERE did = $1",
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
