use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DriverLicenceEntity, IngestionStatus, RucEntity, VehicleEntity};

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    driver_licence_number: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, driver_licence_number)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET driver_licence_number = EXCLUDED.driver_licence_number
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        driver_licence_number,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_driver_licence(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &DriverLicenceEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO driver_licences (citizen_id, licence_number, full_name, licence_class, expiry_date, conditions)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, licence_number) DO UPDATE SET
              full_name = EXCLUDED.full_name,
              licence_class = EXCLUDED.licence_class,
              expiry_date = EXCLUDED.expiry_date,
              conditions = EXCLUDED.conditions
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.licence_number,
        e.full_name,
        e.licence_class,
        e.expiry_date,
        e.conditions,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_vehicle(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &VehicleEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO vehicles (citizen_id, registration, make, model, year, fuel_type, registration_expiry)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, registration) DO UPDATE SET
              make = EXCLUDED.make,
              model = EXCLUDED.model,
              year = EXCLUDED.year,
              fuel_type = EXCLUDED.fuel_type,
              registration_expiry = EXCLUDED.registration_expiry
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.registration,
        e.make,
        e.model,
        e.year,
        e.fuel_type,
        e.registration_expiry,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_ruc(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &RucEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO ruc_records (citizen_id, vehicle_rego, licence_type, expiry_date, units_remaining)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, vehicle_rego) DO UPDATE SET
              licence_type = EXCLUDED.licence_type,
              expiry_date = EXCLUDED.expiry_date,
              units_remaining = EXCLUDED.units_remaining
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.vehicle_rego,
        e.licence_type,
        e.expiry_date,
        e.units_remaining,
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
