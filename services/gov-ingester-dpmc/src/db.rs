use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{HonoursEntity,EngagementsEntity,IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub dpmc_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    dpmc_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, dpmc_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET dpmc_id = EXCLUDED.dpmc_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        dpmc_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_honours(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &HonoursEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO dpmc_honours (citizen_id, award_year, award, status)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, award_year) DO UPDATE SET
              award_year = EXCLUDED.award_year, award = EXCLUDED.award, status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.award_year,
        e.award,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_engagements(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &EngagementsEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO dpmc_engagements (citizen_id, event_name, event_date, location)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, event_name) DO UPDATE SET
              event_name = EXCLUDED.event_name, event_date = EXCLUDED.event_date, location = EXCLUDED.location
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.event_name,
        e.event_date,
        e.location,
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
