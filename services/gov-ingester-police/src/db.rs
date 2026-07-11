use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{InfringementEntity, IngestionStatus, ReportEntity};

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

pub async fn upsert_infringement(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &InfringementEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO police_infringements (citizen_id, ticket_number, offense_type, status, amount, issue_date, location, demerit_points, description)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           ON CONFLICT (citizen_id, ticket_number) DO UPDATE SET
              offense_type = EXCLUDED.offense_type,
              status = EXCLUDED.status,
              amount = EXCLUDED.amount,
              issue_date = EXCLUDED.issue_date,
              location = EXCLUDED.location,
              demerit_points = EXCLUDED.demerit_points,
              description = EXCLUDED.description
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.ticket_number,
        e.offense_type,
        e.status,
        e.amount,
        e.issue_date,
        e.location,
        e.demerit_points,
        e.description,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_report(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ReportEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO police_reports (citizen_id, report_number, report_type, status, filed_date, description)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, report_number) DO UPDATE SET
              report_type = EXCLUDED.report_type,
              status = EXCLUDED.status,
              filed_date = EXCLUDED.filed_date,
              description = EXCLUDED.description
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.report_number,
        e.report_type,
        e.status,
        e.filed_date,
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
