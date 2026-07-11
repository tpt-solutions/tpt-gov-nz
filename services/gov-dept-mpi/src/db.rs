use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub mpi_id: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RegistrationRow {
    pub id: Uuid,
    pub nzbn: String,
    pub business_name: String,
    pub r#type: String,
    pub status: String,
    pub registered_date: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CertificationRow {
    pub id: Uuid,
    pub cert_number: String,
    pub category: String,
    pub issued_date: chrono::NaiveDate,
    pub expires_date: chrono::NaiveDate,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, mpi_id FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_registrations(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<RegistrationRow>> {
    sqlx::query_as!(
        RegistrationRow,
        "SELECT id, nzbn, business_name, type, status, registered_date \
         FROM mpi_registrations WHERE citizen_id = $1 ORDER BY registered_date DESC",
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_certifications(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<CertificationRow>> {
    sqlx::query_as!(
        CertificationRow,
        "SELECT id, cert_number, category, issued_date, expires_date \
         FROM mpi_certifications WHERE citizen_id = $1 ORDER BY issued_date DESC",
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
