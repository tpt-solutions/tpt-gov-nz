use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{BusinessEntity, DirectorshipEntity, IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub person_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    person_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, person_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET person_id = EXCLUDED.person_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        person_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_business(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &BusinessEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO mbie_business_registrations (citizen_id, nzbn, entity_name, entity_type, status, registered_date)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, nzbn) DO UPDATE SET
              entity_name = EXCLUDED.entity_name,
              entity_type = EXCLUDED.entity_type,
              status = EXCLUDED.status,
              registered_date = EXCLUDED.registered_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.nzbn,
        e.entity_name,
        e.entity_type,
        e.status,
        e.registered_date,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_directorship(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &DirectorshipEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO mbie_directorships (citizen_id, nzbn, entity_name, role, appointed_date)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, nzbn, appointed_date) DO UPDATE SET
              entity_name = EXCLUDED.entity_name,
              role = EXCLUDED.role
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.nzbn,
        e.entity_name,
        e.role,
        e.appointed_date,
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
