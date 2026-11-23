use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub maritime_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VesselsRow {

    pub vessel_name: String,
    pub flag: String,
    pub status: String,
}

pub async fn fetch_vessels(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<VesselsRow>> {
    sqlx::query_as!(
        VesselsRow,
        "SELECT vessel_name, flag, status FROM maritime_vessels WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct IncidentsRow {

    pub reference: String,
    pub incident_type: String,
    pub incident_date: chrono::NaiveDate,
    pub status: String,
}

pub async fn fetch_incidents(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<IncidentsRow>> {
    sqlx::query_as!(
        IncidentsRow,
        "SELECT reference, incident_type, incident_date, status FROM maritime_incidents WHERE citizen_id = $1 ORDER BY created_at DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, maritime_id FROM citizens WHERE did = $1",
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
