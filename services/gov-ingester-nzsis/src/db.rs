use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{MandatesEntity,ThreatsEntity,IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub nzsis_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    nzsis_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, nzsis_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET nzsis_id = EXCLUDED.nzsis_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        nzsis_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_mandates(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &MandatesEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO nzsis_mandates (citizen_id, reference, agency, status, issued_date)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, reference) DO UPDATE SET
              reference = EXCLUDED.reference, agency = EXCLUDED.agency, status = EXCLUDED.status, issued_date = EXCLUDED.issued_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.reference,
        e.agency,
        e.status,
        e.issued_date,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_threats(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ThreatsEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO nzsis_threats (citizen_id, reference, category, status, assessed_date)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, reference) DO UPDATE SET
              reference = EXCLUDED.reference, category = EXCLUDED.category, status = EXCLUDED.status, assessed_date = EXCLUDED.assessed_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.reference,
        e.category,
        e.status,
        e.assessed_date,
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
