use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{IngestionStatus, QualificationEntity, TranscriptEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub nsn: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    nsn: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, nsn)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET nsn = EXCLUDED.nsn
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        nsn,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_qualification(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &QualificationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO nzqa_qualifications (citizen_id, qualification_id, title, level, awarded_date, provider)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, qualification_id) DO UPDATE SET
              title = EXCLUDED.title,
              level = EXCLUDED.level,
              awarded_date = EXCLUDED.awarded_date,
              provider = EXCLUDED.provider
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.qualification_id,
        e.title,
        e.level,
        e.awarded_date,
        e.provider,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_transcript(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &TranscriptEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO nzqa_transcripts (citizen_id, record_summary, total_credits, credit_summary)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id) DO UPDATE SET
              record_summary = EXCLUDED.record_summary,
              total_credits = EXCLUDED.total_credits,
              credit_summary = EXCLUDED.credit_summary
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.record_summary,
        e.total_credits,
        e.credit_summary,
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
