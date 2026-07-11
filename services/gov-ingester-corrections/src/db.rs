use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CaseEntity, IngestionStatus, ProbationEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub corrections_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    corrections_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, corrections_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET corrections_id = EXCLUDED.corrections_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        corrections_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_probation(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ProbationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO corrections_probation (citizen_id, status, officer_name, next_report_date, location)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id) DO UPDATE SET
              status = EXCLUDED.status,
              officer_name = EXCLUDED.officer_name,
              next_report_date = EXCLUDED.next_report_date,
              location = EXCLUDED.location
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.status,
        e.officer_name,
        e.next_report_date,
        e.location,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_case(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &CaseEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO corrections_case (citizen_id, case_number, sentence_type, start_date, end_date, summary)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, case_number) DO UPDATE SET
              sentence_type = EXCLUDED.sentence_type,
              start_date = EXCLUDED.start_date,
              end_date = EXCLUDED.end_date,
              summary = EXCLUDED.summary
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.case_number,
        e.sentence_type,
        e.start_date,
        e.end_date,
        e.summary,
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
