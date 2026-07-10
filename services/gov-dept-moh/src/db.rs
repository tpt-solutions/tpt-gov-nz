use sqlx::PgPool;
use uuid::Uuid;

/// Raw DB rows — internal only, never exposed in API responses

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub nhi: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GpEnrolmentRow {
    pub practice_name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PrescriptionRow {
    pub id: Uuid,
    pub medication: String,
    pub dose: String,
    pub repeats_total: i32,
    pub repeats_remaining: i32,
    pub issued_at: chrono::NaiveDate,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AppointmentRow {
    pub id: Uuid,
    pub provider: String,
    pub appt_date: chrono::DateTime<chrono::Utc>,
    pub r#type: String,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VaccinationRow {
    pub id: Uuid,
    pub vaccine: String,
    pub vaccine_date: chrono::NaiveDate,
    pub due_for_booster: bool,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, nhi FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_gp_enrolment(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<GpEnrolmentRow>> {
    sqlx::query_as!(
        GpEnrolmentRow,
        "SELECT practice_name, address, phone FROM gp_enrolments WHERE citizen_id = $1 LIMIT 1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_prescriptions(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<PrescriptionRow>> {
    sqlx::query_as!(
        PrescriptionRow,
        r#"SELECT id, medication, dose, repeats_total, repeats_remaining, issued_at
           FROM prescriptions
           WHERE citizen_id = $1
           ORDER BY issued_at DESC"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_appointments(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<AppointmentRow>> {
    sqlx::query_as!(
        AppointmentRow,
        r#"SELECT id, provider, appt_date, type, status
           FROM appointments
           WHERE citizen_id = $1
           ORDER BY appt_date ASC"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_vaccinations(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Vec<VaccinationRow>> {
    sqlx::query_as!(
        VaccinationRow,
        r#"SELECT id, vaccine, vaccine_date, due_for_booster
           FROM vaccinations
           WHERE citizen_id = $1
           ORDER BY vaccine_date DESC"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_appointment(
    pool: &PgPool,
    citizen_id: Uuid,
    provider: &str,
    appt_date: chrono::DateTime<chrono::Utc>,
    appt_type: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO appointments (citizen_id, provider, appt_date, type, status)
           VALUES ($1, $2, $3, $4, 'booked')"#,
        citizen_id,
        provider,
        appt_date,
        appt_type,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Decrement repeats_remaining for a prescription (repeat request). Returns the new
/// remaining count, or None if the prescription does not exist / has no repeats left.
pub async fn decrement_prescription_repeat(
    pool: &PgPool,
    prescription_id: Uuid,
    citizen_id: Uuid,
) -> sqlx::Result<Option<i32>> {
    let row = sqlx::query!(
        r#"UPDATE prescriptions
           SET repeats_remaining = repeats_remaining - 1
           WHERE id = $1 AND citizen_id = $2 AND repeats_remaining > 0
           RETURNING repeats_remaining"#,
        prescription_id,
        citizen_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.repeats_remaining))
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
