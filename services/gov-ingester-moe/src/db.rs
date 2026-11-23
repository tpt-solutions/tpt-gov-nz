use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{EnrolmentEntity, IngestionStatus, StudentSupportEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub moe_id: String,
}

pub async fn upsert_citizen(pool: &PgPool, did: &str, moe_id: &str) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, moe_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET moe_id = EXCLUDED.moe_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        moe_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_enrolment(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &EnrolmentEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO moe_enrolment (citizen_id, school, year_level, status)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id) DO UPDATE SET
              school = EXCLUDED.school, year_level = EXCLUDED.year_level, status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.school,
        e.year_level,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_student_support(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &StudentSupportEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO moe_student_support (citizen_id, service, status, next_review)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, service) DO UPDATE SET
              service = EXCLUDED.service, status = EXCLUDED.status, next_review = EXCLUDED.next_review
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.service,
        e.status,
        e.next_review,
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
