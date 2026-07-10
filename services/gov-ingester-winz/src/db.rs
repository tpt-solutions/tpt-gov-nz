use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{BenefitEntity, IngestionStatus, PaymentEntity};

/// Upsert a citizen. Returns `(citizen_id, inserted)` — `inserted` is true when this
/// run created the row (vs. updated an existing one), tracked via Postgres `xmax`.
pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    client_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, client_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET client_id = EXCLUDED.client_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        client_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_benefit(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &BenefitEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO benefits
              (citizen_id, benefit_type, weekly_amount, start_date, review_date, status)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, benefit_type) DO UPDATE SET
              weekly_amount = EXCLUDED.weekly_amount,
              start_date = EXCLUDED.start_date,
              review_date = EXCLUDED.review_date,
              status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.benefit_type,
        e.weekly_amount,
        e.start_date,
        e.review_date,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_payment(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &PaymentEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO payments (citizen_id, benefit_type, payment_date, amount, method)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, benefit_type, payment_date, amount) DO UPDATE SET
              method = EXCLUDED.method
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.benefit_type,
        e.payment_date,
        e.amount,
        e.method,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

/// Persist an ingestion run to the audit log. `status` is encoded as text.
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

/// Most recent ingestion run (for diagnostics / health).
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
