use sqlx::PgPool;
use uuid::Uuid;

/// Raw DB rows — internal only, never exposed in API responses

#[derive(Debug, sqlx::FromRow)]
pub struct CitizenRow {
    pub id: Uuid,
    pub did: String,
    pub ird_number: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TaxAssessmentRow {
    pub assessment_year: i32,
    pub tax_code: String,
    pub total_income: sqlx::types::Decimal,
    pub taxable_income: sqlx::types::Decimal,
    pub tax_liability: sqlx::types::Decimal,
    pub tax_paid: sqlx::types::Decimal,
    pub tax_refund_due: sqlx::types::Decimal,
    pub tax_owing: sqlx::types::Decimal,
    pub assessment_status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct IncomeRow {
    pub assessment_year: i32,
    pub employment_income: Option<sqlx::types::Decimal>,
    pub self_employment_income: Option<sqlx::types::Decimal>,
    pub rental_income: Option<sqlx::types::Decimal>,
    pub other_income: Option<sqlx::types::Decimal>,
    pub total_deductions: Option<sqlx::types::Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GstRegistrationRow {
    pub registered: bool,
    pub gst_number: Option<String>,
    pub filing_frequency: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GstPeriodRow {
    pub id: Uuid,
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
    pub filing_due: chrono::NaiveDate,
    pub status: String,
    pub sales_income: Option<sqlx::types::Decimal>,
    pub gst_on_sales: Option<sqlx::types::Decimal>,
    pub gst_on_purchases: Option<sqlx::types::Decimal>,
    pub refund_or_payment: Option<sqlx::types::Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct KiwiSaverRow {
    pub membership_status: String,
    pub contribution_rate: sqlx::types::Decimal,
    pub employer_contribution_rate: Option<sqlx::types::Decimal>,
    pub scheme: Option<String>,
    pub total_balance: Option<sqlx::types::Decimal>,
    pub last_contribution_date: Option<chrono::NaiveDate>,
    pub government_contribution_eligible: bool,
    pub first_home_buyer_eligible: Option<bool>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct WffRow {
    pub eligible: bool,
    pub number_of_dependant_children: i32,
    pub income_threshold: sqlx::types::Decimal,
    pub family_tax_credit: Option<sqlx::types::Decimal>,
    pub in_work_tax_credit: Option<sqlx::types::Decimal>,
    pub best_start_payment: Option<sqlx::types::Decimal>,
    pub minimum_family_tax_credit: Option<sqlx::types::Decimal>,
    pub payment_frequency: Option<String>,
    pub next_review_date: Option<chrono::NaiveDate>,
}

pub async fn resolve_by_did(pool: &PgPool, did: &str) -> sqlx::Result<Option<CitizenRow>> {
    sqlx::query_as!(
        CitizenRow,
        "SELECT id, did, ird_number FROM citizens WHERE did = $1",
        did
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_current_tax_assessment(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<TaxAssessmentRow>> {
    sqlx::query_as!(
        TaxAssessmentRow,
        r#"SELECT assessment_year, tax_code, total_income, taxable_income,
                  tax_liability, tax_paid, tax_refund_due, tax_owing, assessment_status
           FROM tax_assessments
           WHERE citizen_id = $1
           ORDER BY assessment_year DESC
           LIMIT 1"#,
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_all_tax_assessments(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<TaxAssessmentRow>> {
    sqlx::query_as!(
        TaxAssessmentRow,
        r#"SELECT assessment_year, tax_code, total_income, taxable_income,
                  tax_liability, tax_paid, tax_refund_due, tax_owing, assessment_status
           FROM tax_assessments
           WHERE citizen_id = $1
           ORDER BY assessment_year DESC"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_income_for_year(
    pool: &PgPool,
    citizen_id: Uuid,
    year: i32,
) -> sqlx::Result<Option<IncomeRow>> {
    sqlx::query_as!(
        IncomeRow,
        r#"SELECT assessment_year, employment_income, self_employment_income,
                  rental_income, other_income, total_deductions
           FROM income_records
           WHERE citizen_id = $1 AND assessment_year = $2"#,
        citizen_id,
        year
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_gst_registration(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Option<GstRegistrationRow>> {
    sqlx::query_as!(
        GstRegistrationRow,
        "SELECT registered, gst_number, filing_frequency FROM gst_registrations WHERE citizen_id = $1",
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_gst_periods(
    pool: &PgPool,
    citizen_id: Uuid,
) -> sqlx::Result<Vec<GstPeriodRow>> {
    sqlx::query_as!(
        GstPeriodRow,
        r#"SELECT id, period_start, period_end, filing_due, status,
                  sales_income, gst_on_sales, gst_on_purchases, refund_or_payment
           FROM gst_periods
           WHERE citizen_id = $1
           ORDER BY period_start DESC
           LIMIT 12"#,
        citizen_id
    )
    .fetch_all(pool)
    .await
}

pub async fn fetch_kiwisaver(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<KiwiSaverRow>> {
    sqlx::query_as!(
        KiwiSaverRow,
        r#"SELECT membership_status, contribution_rate, employer_contribution_rate,
                  scheme, total_balance, last_contribution_date,
                  government_contribution_eligible, first_home_buyer_eligible
           FROM kiwisaver_memberships
           WHERE citizen_id = $1"#,
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn fetch_wff(pool: &PgPool, citizen_id: Uuid) -> sqlx::Result<Option<WffRow>> {
    sqlx::query_as!(
        WffRow,
        r#"SELECT eligible, number_of_dependant_children, income_threshold,
                  family_tax_credit, in_work_tax_credit, best_start_payment,
                  minimum_family_tax_credit, payment_frequency, next_review_date
           FROM wff_entitlements
           WHERE citizen_id = $1"#,
        citizen_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_kiwisaver_rate(
    pool: &PgPool,
    citizen_id: Uuid,
    new_rate: f64,
) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE kiwisaver_memberships SET contribution_rate = $1, updated_at = NOW() WHERE citizen_id = $2",
        sqlx::types::Decimal::try_from(new_rate).unwrap(),
        citizen_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn file_gst_return(
    pool: &PgPool,
    citizen_id: Uuid,
    period_id: Uuid,
    sales_income: f64,
    gst_on_sales: f64,
    gst_on_purchases: f64,
) -> sqlx::Result<()> {
    let refund_or_payment = gst_on_purchases - gst_on_sales;
    sqlx::query!(
        r#"UPDATE gst_periods
           SET status = 'filed', sales_income = $1, gst_on_sales = $2,
               gst_on_purchases = $3, refund_or_payment = $4, filed_at = NOW()
           WHERE id = $5 AND citizen_id = $6 AND status IN ('due', 'overdue')"#,
        sqlx::types::Decimal::try_from(sales_income).unwrap(),
        sqlx::types::Decimal::try_from(gst_on_sales).unwrap(),
        sqlx::types::Decimal::try_from(gst_on_purchases).unwrap(),
        sqlx::types::Decimal::try_from(refund_or_payment).unwrap(),
        period_id,
        citizen_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn log_action(
    pool: &PgPool,
    citizen_id: Uuid,
    action_type: &str,
    parameters: serde_json::Value,
    performed_by: &str,
    ai_level: Option<&str>,
    result_success: bool,
    result_message: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"INSERT INTO actions_log (citizen_id, action_type, parameters, performed_by, ai_level, result_success, result_message)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        citizen_id,
        action_type,
        parameters,
        performed_by,
        ai_level,
        result_success,
        result_message
    )
    .execute(pool)
    .await?;
    Ok(())
}
