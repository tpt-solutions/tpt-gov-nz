use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub client_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClaimRow {
    pub claim_number: String,
    pub claim_type: String,
    pub status: String,
    pub injury_date: chrono::NaiveDate,
    pub description: String,
    pub weekly_compensation: Option<f64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EntitlementRow {
    pub has_entitlement: bool,
    pub r#type: Option<String>,
    pub weekly_amount: Option<f64>,
    pub remaining_weeks: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RehabilitationRow {
    pub plan_id: String,
    pub description: String,
    pub status: String,
    pub provider: Option<String>,
    pub next_review: Option<chrono::NaiveDate>,
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

pub async fn fetch_claims(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<ClaimRow>> {
    sqlx::query_as!(
        ClaimRow,
        "SELECT claim_number, claim_type, status, injury_date, description, weekly_compensation FROM acc_claims WHERE citizen_id = $1 ORDER BY injury_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_entitlement(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<EntitlementRow>> {
    sqlx::query_as!(
        EntitlementRow,
        "SELECT has_entitlement, type, weekly_amount, remaining_weeks FROM acc_entitlements WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_rehabilitation(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<RehabilitationRow>> {
    sqlx::query_as!(
        RehabilitationRow,
        "SELECT plan_id, description, status, provider, next_review FROM acc_rehabilitation WHERE citizen_id = $1 ORDER BY next_review NULLS LAST",
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
