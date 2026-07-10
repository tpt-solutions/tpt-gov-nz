use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{BirthCertEntity, CitizenshipEntity, IngestionStatus, PassportEntity};

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    passport_number: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, passport_number)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET passport_number = EXCLUDED.passport_number
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        passport_number,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_passport(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &PassportEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO passports (citizen_id, passport_number, expiry_date, renewable)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, passport_number) DO UPDATE SET
              expiry_date = EXCLUDED.expiry_date,
              renewable = EXCLUDED.renewable
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.passport_number,
        e.expiry_date,
        e.renewable,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_birth_cert(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &BirthCertEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO birth_certs (citizen_id, certificate_number, date_of_birth, place_of_birth, parents)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, certificate_number) DO UPDATE SET
              date_of_birth = EXCLUDED.date_of_birth,
              place_of_birth = EXCLUDED.place_of_birth,
              parents = EXCLUDED.parents
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.certificate_number,
        e.date_of_birth,
        e.place_of_birth,
        e.parents,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_citizenship(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &CitizenshipEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO citizenship_records (citizen_id, status, certificate_number, granted_at)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, status) DO UPDATE SET
              certificate_number = EXCLUDED.certificate_number,
              granted_at = EXCLUDED.granted_at
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.status,
        e.certificate_number,
        e.granted_at,
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
