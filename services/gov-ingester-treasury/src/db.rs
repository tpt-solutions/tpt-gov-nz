use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{BudgetEntity,EconomicOutlookEntity,IngestionStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub treasury_id: String,
}

pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    treasury_id: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, treasury_id)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET treasury_id = EXCLUDED.treasury_id
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        treasury_id,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_budget(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &BudgetEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO treasury_budget (citizen_id, fiscal_year, portfolio, appropriation, amount)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, fiscal_year) DO UPDATE SET
              fiscal_year = EXCLUDED.fiscal_year, portfolio = EXCLUDED.portfolio, appropriation = EXCLUDED.appropriation, amount = EXCLUDED.amount
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.fiscal_year,
        e.portfolio,
        e.appropriation,
        e.amount,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_economic_outlook(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &EconomicOutlookEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO treasury_economic_outlook (citizen_id, forecast_year, gdp_growth_pct, inflation_pct, net_debt_pct)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id) DO UPDATE SET
              forecast_year = EXCLUDED.forecast_year, gdp_growth_pct = EXCLUDED.gdp_growth_pct, inflation_pct = EXCLUDED.inflation_pct, net_debt_pct = EXCLUDED.net_debt_pct
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.forecast_year,
        e.gdp_growth_pct,
        e.inflation_pct,
        e.net_debt_pct,
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
