use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{FireSafetyEntity, IncidentsEntity, IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub fenz_id: String,
}

pub async fn upsert_citizen(pool: &PgPool, did: &str, fenz_id: &str) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, fenz_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET fenz_id = EXCLUDED.fenz_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        fenz_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_fire_safety(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &FireSafetyEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO fenz_fire_safety (citizen_id, property, grade, last_inspection)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id) DO UPDATE SET
              property = EXCLUDED.property, grade = EXCLUDED.grade, last_inspection = EXCLUDED.last_inspection
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.property,
        e.grade,
        e.last_inspection,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_incidents(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &IncidentsEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO fenz_incidents (citizen_id, reference, incident_type, incident_date, status)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, reference) DO UPDATE SET
              reference = EXCLUDED.reference, incident_type = EXCLUDED.incident_type, incident_date = EXCLUDED.incident_date, status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.reference,
        e.incident_type,
        e.incident_date,
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

pub async fn record_ingestion_run(pool: &PgPool, run: &IngestionRunRecord<'_>) -> sqlx::Result<()> {
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
