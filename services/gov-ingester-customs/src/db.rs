use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DeclarationEntity, IngestionStatus, TravelEntity};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub traveller_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    traveller_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, traveller_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET traveller_id = EXCLUDED.traveller_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        traveller_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_travel(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &TravelEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO customs_travel (citizen_id, passport_number, last_arrival, arrival_port, frequent_traveller)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id) DO UPDATE SET
              passport_number = EXCLUDED.passport_number,
              last_arrival = EXCLUDED.last_arrival,
              arrival_port = EXCLUDED.arrival_port,
              frequent_traveller = EXCLUDED.frequent_traveller
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.passport_number,
        e.last_arrival,
        e.arrival_port,
        e.frequent_traveller,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_declaration(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &DeclarationEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO customs_declarations (citizen_id, declaration_id, date, country_from, goods_declared, status)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (citizen_id, declaration_id) DO UPDATE SET
              date = EXCLUDED.date,
              country_from = EXCLUDED.country_from,
              goods_declared = EXCLUDED.goods_declared,
              status = EXCLUDED.status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.declaration_id,
        e.date,
        e.country_from,
        e.goods_declared,
        e.status,
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
