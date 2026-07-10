use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    GstEntity, IncomeEntity, IngestionStatus, KiwiSaverEntity, TaxEntity, WffEntity,
};

/// Upsert a citizen. Returns `(citizen_id, inserted)` — `inserted` is true when this
/// run created the row (vs. updated an existing one), tracked via Postgres `xmax`.
pub async fn upsert_citizen(
    pool: &PgPool,
    did: &str,
    ird_number: &str,
) -> sqlx::Result<(Uuid, bool)> {
    let row = sqlx::query!(
        r#"INSERT INTO citizens (did, ird_number)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET ird_number = EXCLUDED.ird_number
           RETURNING id, (xmax = 0) AS inserted"#,
        did,
        ird_number,
    )
    .fetch_one(pool)
    .await?;
    Ok((row.id, row.inserted.unwrap_or(false)))
}

pub async fn upsert_income(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &IncomeEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO income_records
              (citizen_id, assessment_year, employment_income, self_employment_income,
               rental_income, other_income, total_deductions)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (citizen_id, assessment_year) DO UPDATE SET
              employment_income = EXCLUDED.employment_income,
              self_employment_income = EXCLUDED.self_employment_income,
              rental_income = EXCLUDED.rental_income,
              other_income = EXCLUDED.other_income,
              total_deductions = EXCLUDED.total_deductions
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.assessment_year,
        e.employment_income,
        e.self_employment_income,
        e.rental_income,
        e.other_income,
        e.total_deductions,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_tax_assessment(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &TaxEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO tax_assessments
              (citizen_id, assessment_year, tax_code, total_income, taxable_income,
               tax_liability, tax_paid, tax_refund_due, tax_owing, assessment_status)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           ON CONFLICT (citizen_id, assessment_year) DO UPDATE SET
              tax_code = EXCLUDED.tax_code,
              total_income = EXCLUDED.total_income,
              taxable_income = EXCLUDED.taxable_income,
              tax_liability = EXCLUDED.tax_liability,
              tax_paid = EXCLUDED.tax_paid,
              tax_refund_due = EXCLUDED.tax_refund_due,
              tax_owing = EXCLUDED.tax_owing,
              assessment_status = EXCLUDED.assessment_status
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.assessment_year,
        e.tax_code,
        e.total_income,
        e.taxable_income,
        e.tax_liability,
        e.tax_paid,
        e.tax_refund_due,
        e.tax_owing,
        e.assessment_status,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_gst_registration(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &GstEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO gst_registrations (citizen_id, registered, gst_number, filing_frequency)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (citizen_id) DO UPDATE SET
              registered = EXCLUDED.registered,
              gst_number = EXCLUDED.gst_number,
              filing_frequency = EXCLUDED.filing_frequency
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.registered,
        e.gst_number,
        e.filing_frequency,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_kiwisaver(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &KiwiSaverEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO kiwisaver_memberships
              (citizen_id, membership_status, contribution_rate, employer_contribution_rate,
               scheme, total_balance, government_contribution_eligible, first_home_buyer_eligible)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           ON CONFLICT (citizen_id) DO UPDATE SET
              membership_status = EXCLUDED.membership_status,
              contribution_rate = EXCLUDED.contribution_rate,
              employer_contribution_rate = EXCLUDED.employer_contribution_rate,
              scheme = EXCLUDED.scheme,
              total_balance = EXCLUDED.total_balance,
              government_contribution_eligible = EXCLUDED.government_contribution_eligible,
              first_home_buyer_eligible = EXCLUDED.first_home_buyer_eligible
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.membership_status,
        e.contribution_rate,
        e.employer_contribution_rate,
        e.scheme,
        e.total_balance,
        e.government_contribution_eligible,
        e.first_home_buyer_eligible,
    )
    .fetch_one(pool)
    .await?;
    Ok(row.inserted.unwrap_or(false))
}

pub async fn upsert_wff(
    pool: &PgPool,
    citizen_id: Uuid,
    e: &WffEntity,
) -> sqlx::Result<bool> {
    let row = sqlx::query!(
        r#"INSERT INTO wff_entitlements
              (citizen_id, eligible, number_of_dependant_children, income_threshold,
               family_tax_credit, in_work_tax_credit, best_start_payment,
               minimum_family_tax_credit, payment_frequency)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           ON CONFLICT (citizen_id) DO UPDATE SET
              eligible = EXCLUDED.eligible,
              number_of_dependant_children = EXCLUDED.number_of_dependant_children,
              income_threshold = EXCLUDED.income_threshold,
              family_tax_credit = EXCLUDED.family_tax_credit,
              in_work_tax_credit = EXCLUDED.in_work_tax_credit,
              best_start_payment = EXCLUDED.best_start_payment,
              minimum_family_tax_credit = EXCLUDED.minimum_family_tax_credit,
              payment_frequency = EXCLUDED.payment_frequency
           RETURNING (xmax = 0) AS inserted"#,
        citizen_id,
        e.eligible,
        e.number_of_dependant_children,
        e.income_threshold,
        e.family_tax_credit,
        e.in_work_tax_credit,
        e.best_start_payment,
        e.minimum_family_tax_credit,
        e.payment_frequency,
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
