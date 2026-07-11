use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ConcessionEntity, IngestionStatus, PermitEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub doc_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    doc_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, doc_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET doc_id = EXCLUDED.doc_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        doc_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_permit(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &PermitEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO doc_permits (citizen_id, permit_number, activity, location, status, expires_date)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, permit_number) DO UPDATE SET
              activity = EXCLUDED.activity,
              location = EXCLUDED.location,
              status = EXCLUDED.status,
              expires_date = EXCLUDED.expires_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.permit_number,
        e.activity,
        e.location,
        e.status,
        e.expires_date,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_concession(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ConcessionEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO doc_concessions (citizen_id, concession_id, type, holder, start_date, end_date)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, concession_id) DO UPDATE SET
              type = EXCLUDED.type,
              holder = EXCLUDED.holder,
              start_date = EXCLUDED.start_date,
              end_date = EXCLUDED.end_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.concession_id,
        e.r#type,
        e.holder,
        e.start_date,
        e.end_date,
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
