use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{IngestionStatus, ProgrammesEntity, StrategiesEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub mot_id: String,
}

pub async fn upsert_citizen(pool: &PgPool, did: &str, mot_id: &str) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, mot_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET mot_id = EXCLUDED.mot_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        mot_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_strategies(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &StrategiesEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO mot_strategies (citizen_id, title, year, status)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, title) DO UPDATE SET
              title = EXCLUDED.title, year = EXCLUDED.year, status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.title,
        e.year,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_programmes(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ProgrammesEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO mot_programmes (citizen_id, name, budget, status)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, name) DO UPDATE SET
              name = EXCLUDED.name, budget = EXCLUDED.budget, status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.name,
        e.budget,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

/// One ingestion run's outcome, written to `ingestion_runs`.
#[derive(Debug, Clone, Copy)]
pub struct IngestionRunRecord<'a> {
    pub run_id: Uuid,
    pub source: &'a str,
    pub batch_id: Option<&'a str>,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub citizens_processed: i32,
    pub rows_inserted: i32,
    pub rows_updated: i32,
    pub status: IngestionStatus,
    pub error_message: Option<&'a str>,
}

pub async fn record_ingestion_run(
    pool: &PgPool,
    run: &IngestionRunRecord<'_>,
) -> sqlx::Result<()> {
    let IngestionRunRecord {
        run_id,
        source,
        batch_id,
        started_at,
        finished_at,
        citizens_processed,
        rows_inserted,
        rows_updated,
        status,
        error_message,
    } = *run;
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
