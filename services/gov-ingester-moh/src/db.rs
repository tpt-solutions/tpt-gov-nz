use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{AppointmentEntity, GpEntity, IngestionStatus, PrescriptionEntity, VaccinationEntity};

/// Upsert a citizen. Returns `(citizen_id, inserted)`.
pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    nhi: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, nhi)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET nhi = EXCLUDED.nhi
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        nhi,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_gp_enrolment(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &GpEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO gp_enrolments (citizen_id, practice_name, address, phone, enrolled_at)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, practice_name) DO UPDATE SET
              address = EXCLUDED.address,
              phone = EXCLUDED.phone,
              enrolled_at = EXCLUDED.enrolled_at
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.practice_name,
        e.address,
        e.phone,
        e.enrolled_at,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_prescription(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &PrescriptionEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO prescriptions (citizen_id, medication, dose, repeats_total, repeats_remaining, issued_at)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, medication, issued_at) DO UPDATE SET
              dose = EXCLUDED.dose,
              repeats_total = EXCLUDED.repeats_total,
              repeats_remaining = EXCLUDED.repeats_remaining
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.medication,
        e.dose,
        e.repeats_total,
        e.repeats_remaining,
        e.issued_at,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_appointment(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &AppointmentEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO appointments (citizen_id, provider, appt_date, type, status)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, provider, appt_date) DO UPDATE SET
              type = EXCLUDED.type,
              status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.provider,
        e.appt_date,
        e.r#type,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_vaccination(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &VaccinationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO vaccinations (citizen_id, vaccine, vaccine_date, due_for_booster)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, vaccine, vaccine_date) DO UPDATE SET
              due_for_booster = EXCLUDED.due_for_booster
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.vaccine,
        e.vaccine_date,
        e.due_for_booster,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn record_ingestion_run(
    pool: &PgPool,
    run_id: Uuid,
    source: &str,
    batch_id: Option<&str>,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
    citizens_processed: i32,
    rows_inserted: i32,
    rows_updated: i32,
    status: IngestionStatus,
    error_message: Option<&str>,
) -> sqlx::Result<()> {
    let status_str = match status {
        IngestionStatus::Running => "running",
        IngestionStatus::Success => "success",
        IngestionStatus::Failed => "failed",
    };
    sqlx::query!(
        r#"INSERT INTO ingestion_runs
              (id, source, batch_id, run_started_at, run_finished_at, citizens_processed,
               rows_inserted, rows_updated, status, error_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        run_id,
        source,
        batch_id,
        started_at,
        finished_at,
        citizens_processed,
        rows_inserted,
        rows_updated,
        status_str,
        error_message,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn latest_run(pool: &PgPool) -> sqlx::Result<Option<(String, i32, i32, String)>> {
    let row = sqlx::query!(
        r#"SELECT source, citizens_processed, rows_inserted, status
           FROM ingestion_runs ORDER BY run_started_at DESC LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| (r.source, r.citizens_processed, r.rows_inserted, r.status)))
}
