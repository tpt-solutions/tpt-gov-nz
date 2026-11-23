use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub stats_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CensusRow {
    pub census_year: i32,
    pub dwelling_type: String,
    pub household_size: i32,
    pub region: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ProfileRow {
    pub data_summary: String,
    pub record_count: i32,
    pub last_updated: chrono::NaiveDate,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, stats_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_census(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<CensusRow>> {
    sqlx::query_as!(
        CensusRow,
        "SELECT census_year, dwelling_type, household_size, region FROM statsnz_census WHERE citizen_id = $1 ORDER BY census_year DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_profile(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<ProfileRow>> {
    sqlx::query_as!(
        ProfileRow,
        "SELECT data_summary, record_count, last_updated FROM statsnz_profile WHERE citizen_id = $1",
        citizen_id
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
