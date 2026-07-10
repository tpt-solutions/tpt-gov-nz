use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use uuid::Uuid;

/// Result of a single ingestion pass. Carried through to the audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionSummary {
    pub source: String,
    pub batch_id: Option<String>,
    pub citizens_processed: u32,
    pub rows_inserted: u32,
    pub rows_updated: u32,
    pub status: IngestionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IngestionStatus {
    Running,
    Success,
    Failed,
}

/// Typed, schema-aligned view of one citizen, ready to upsert.
/// Produced by the transform layer from [`crate::raw::RawIrdCitizen`].
#[derive(Debug, Clone)]
pub struct CitizenEntity {
    pub did: String,
    pub ird_number: String,
}

#[derive(Debug, Clone)]
pub struct IncomeEntity {
    pub assessment_year: i32,
    pub employment_income: Option<Decimal>,
    pub self_employment_income: Option<Decimal>,
    pub rental_income: Option<Decimal>,
    pub other_income: Option<Decimal>,
    pub total_deductions: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct TaxEntity {
    pub assessment_year: i32,
    pub tax_code: String,
    pub total_income: Decimal,
    pub taxable_income: Decimal,
    pub tax_liability: Decimal,
    pub tax_paid: Decimal,
    pub tax_refund_due: Decimal,
    pub tax_owing: Decimal,
    pub assessment_status: String,
}

#[derive(Debug, Clone)]
pub struct GstEntity {
    pub registered: bool,
    pub gst_number: Option<String>,
    pub filing_frequency: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KiwiSaverEntity {
    pub membership_status: String,
    pub contribution_rate: Decimal,
    pub employer_contribution_rate: Option<Decimal>,
    pub scheme: Option<String>,
    pub total_balance: Option<Decimal>,
    pub government_contribution_eligible: bool,
    pub first_home_buyer_eligible: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct WffEntity {
    pub eligible: bool,
    pub number_of_dependant_children: i32,
    pub income_threshold: Decimal,
    pub family_tax_credit: Option<Decimal>,
    pub in_work_tax_credit: Option<Decimal>,
    pub best_start_payment: Option<Decimal>,
    pub minimum_family_tax_credit: Option<Decimal>,
    pub payment_frequency: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub income: Option<IncomeEntity>,
    pub tax: Option<TaxEntity>,
    pub gst: GstEntity,
    pub kiwisaver: Option<KiwiSaverEntity>,
    pub wff: Option<WffEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
