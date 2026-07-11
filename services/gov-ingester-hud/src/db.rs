use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{ApplicationEntity, IngestionStatus, MaintenanceRequestEntity, TenancyEntity};

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

pub async fn upsert_application(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &ApplicationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO hud_applications (citizen_id, application_number, application_type, status, priority_band, bedrooms_needed, submitted_date)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, application_number) DO UPDATE SET
              application_type = EXCLUDED.application_type,
              status = EXCLUDED.status,
              priority_band = EXCLUDED.priority_band,
              bedrooms_needed = EXCLUDED.bedrooms_needed,
              submitted_date = EXCLUDED.submitted_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.application_number,
        e.application_type,
        e.status,
        e.priority_band,
        e.bedrooms_needed,
        e.submitted_date,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_tenancy(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &TenancyEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO hud_tenancies (citizen_id, tenancy_id, property_address, weekly_rent, income_related_rent, start_date, status)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, tenancy_id) DO UPDATE SET
              property_address = EXCLUDED.property_address,
              weekly_rent = EXCLUDED.weekly_rent,
              income_related_rent = EXCLUDED.income_related_rent,
              start_date = EXCLUDED.start_date,
              status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.tenancy_id,
        e.property_address,
        e.weekly_rent,
        e.income_related_rent,
        e.start_date,
        e.status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_maintenance_request(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &MaintenanceRequestEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO hud_maintenance_requests (citizen_id, request_number, category, status, description, requested_date)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, request_number) DO UPDATE SET
              category = EXCLUDED.category,
              status = EXCLUDED.status,
              description = EXCLUDED.description,
              requested_date = EXCLUDED.requested_date
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.request_number,
        e.category,
        e.status,
        e.description,
        e.requested_date,
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
