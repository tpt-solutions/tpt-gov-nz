use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{EntitlementEntity, IngestionStatus, RehabilitationEntity, ClaimEntity};

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

pub async fn upsert_claim(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ClaimEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO acc_claims (citizen_id, claim_number, claim_type, status, injury_date, description, weekly_compensation)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, claim_number) DO UPDATE SET
              claim_type = EXCLUDED.claim_type,
              status = EXCLUDED.status,
              injury_date = EXCLUDED.injury_date,
              description = EXCLUDED.description,
              weekly_compensation = EXCLUDED.weekly_compensation
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.claim_number,
        e.claim_type,
        e.status,
        e.injury_date,
        e.description,
        e.weekly_compensation,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_entitlement(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &EntitlementEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO acc_entitlements (citizen_id, has_entitlement, type, weekly_amount, remaining_weeks)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id) DO UPDATE SET
              has_entitlement = EXCLUDED.has_entitlement,
              type = EXCLUDED.type,
              weekly_amount = EXCLUDED.weekly_amount,
              remaining_weeks = EXCLUDED.remaining_weeks
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.has_entitlement,
        e.r#type,
        e.weekly_amount,
        e.remaining_weeks,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_rehabilitation(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &RehabilitationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO acc_rehabilitation (citizen_id, plan_id, description, status, provider, next_review)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, plan_id) DO UPDATE SET
              description = EXCLUDED.description,
              status = EXCLUDED.status,
              provider = EXCLUDED.provider,
              next_review = EXCLUDED.next_review
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.plan_id,
        e.description,
        e.status,
        e.provider,
        e.next_review,
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
