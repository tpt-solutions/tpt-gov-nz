use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CourtRecordEntity, DisputeEntity, FineEntity, IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub client_number: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    client_number: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, client_number)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET client_number = EXCLUDED.client_number
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        client_number,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_fine(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &FineEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO moj_fines (citizen_id, fine_number, fine_type, status, amount, offense_date, due_date, description)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (citizen_id, fine_number) DO UPDATE SET
              fine_type = EXCLUDED.fine_type,
              status = EXCLUDED.status,
              amount = EXCLUDED.amount,
              offense_date = EXCLUDED.offense_date,
              due_date = EXCLUDED.due_date,
              description = EXCLUDED.description
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.fine_number,
        e.fine_type,
        e.status,
        e.amount,
        e.offense_date,
        e.due_date,
        e.description,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_dispute(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &DisputeEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO moj_disputes (citizen_id, dispute_number, claim_type, status, amount_claimed, hearing_date, description)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, dispute_number) DO UPDATE SET
              claim_type = EXCLUDED.claim_type,
              status = EXCLUDED.status,
              amount_claimed = EXCLUDED.amount_claimed,
              hearing_date = EXCLUDED.hearing_date,
              description = EXCLUDED.description
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.dispute_number,
        e.claim_type,
        e.status,
        e.amount_claimed,
        e.hearing_date,
        e.description,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_court_record(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &CourtRecordEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO moj_court_records (citizen_id, case_number, case_type, status, next_hearing_date, description)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, case_number) DO UPDATE SET
              case_type = EXCLUDED.case_type,
              status = EXCLUDED.status,
              next_hearing_date = EXCLUDED.next_hearing_date,
              description = EXCLUDED.description
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.case_number,
        e.case_type,
        e.status,
        e.next_hearing_date,
        e.description,
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
