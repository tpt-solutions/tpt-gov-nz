use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{IngestionStatus, OwnershipEntity, TitleEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub customer_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    customer_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, customer_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET customer_id = EXCLUDED.customer_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        customer_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_title(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &TitleEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO linz_titles (citizen_id, title_number, property_address, land_area_sqm, estate_type)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, title_number) DO UPDATE SET
              property_address = EXCLUDED.property_address,
              land_area_sqm = EXCLUDED.land_area_sqm,
              estate_type = EXCLUDED.estate_type
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.title_number,
        e.property_address,
        e.land_area_sqm,
        e.estate_type,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_ownership(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &OwnershipEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO linz_ownership (citizen_id, title_number, ownership_share, registered_owners)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id, title_number) DO UPDATE SET
              ownership_share = EXCLUDED.ownership_share,
              registered_owners = EXCLUDED.registered_owners
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.title_number,
        e.ownership_share,
        e.registered_owners,
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
