use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub passport_number: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PassportRow {
    pub passport_number: Option<String>,
    pub expiry_date: chrono::NaiveDate,
    pub renewable: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BirthCertRow {
    pub certificate_number: String,
    pub date_of_birth: chrono::NaiveDate,
    pub place_of_birth: String,
    pub parents: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenshipRow {
    pub status: String,
    pub certificate_number: Option<String>,
    pub granted_at: Option<chrono::NaiveDate>,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, passport_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_passport(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<PassportRow>> {
    sqlx::query_as!(
        PassportRow,
        "SELECT passport_number, expiry_date, renewable FROM passports WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_birth_cert(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<BirthCertRow>> {
    sqlx::query_as!(
        BirthCertRow,
        "SELECT certificate_number, date_of_birth, place_of_birth, parents FROM birth_certs WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_citizenship(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<CitizenshipRow>> {
    sqlx::query_as!(
        CitizenshipRow,
        "SELECT status, certificate_number, granted_at FROM citizenship_records WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_action_request(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    detail: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO actions_log (citizen_id, action_type, parameters, performed_by, result_success, result_message) \
         VALUES ($1, $2, $3, 'citizen', true, $4)",
        citizen_id,
        action_type,
        serde_json::json!({ "detail": detail }),
        detail,
    )
    .execute(pool)
    .await?;
    Ok(())
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
