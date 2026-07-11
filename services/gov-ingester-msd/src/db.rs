use chrono::{DateTime, Utc};
use sqlx::PgPool;
use sqlx::types::Decimal;
use uuid::Uuid;

use crate::models::{CaseEventEntity, IngestionStatus, StudyLinkEntity};

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

pub async fn upsert_studylink(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &StudyLinkEntity,
) -> sqlx::Result<bool> {
    let loan_balance: Option<Decimal> = e.loan_balance.and_then(Decimal::from_f64_retain);
    let weekly_amount: Option<Decimal> = e.weekly_amount.and_then(Decimal::from_f64_retain);
    let row = sqlx::query!(
        r#"INSERT INTO msd_studylink (citizen_id, has_student_loan, loan_balance, repayment_plan, has_allowance, allowance_type, next_payment_date, weekly_amount)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (citizen_id) DO UPDATE SET
              has_student_loan = EXCLUDED.has_student_loan,
              loan_balance = EXCLUDED.loan_balance,
              repayment_plan = EXCLUDED.repayment_plan,
              has_allowance = EXCLUDED.has_allowance,
              allowance_type = EXCLUDED.allowance_type,
              next_payment_date = EXCLUDED.next_payment_date,
              weekly_amount = EXCLUDED.weekly_amount
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.has_student_loan,
        loan_balance,
        e.repayment_plan,
        e.has_allowance,
        e.allowance_type,
        e.next_payment_date,
        weekly_amount,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_case_event(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &CaseEventEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO msd_case_history (citizen_id, event_id, event_date, service_line, summary)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (citizen_id, event_id) DO UPDATE SET
              event_date = EXCLUDED.event_date,
              service_line = EXCLUDED.service_line,
              summary = EXCLUDED.summary
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.event_id,
        e.event_date,
        e.service_line,
        e.summary,
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
