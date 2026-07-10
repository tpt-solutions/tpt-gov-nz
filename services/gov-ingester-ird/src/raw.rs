//! Raw IRD legacy data format.
//!
//! This mirrors the shape of a batch file as produced by IRD's legacy systems
//! (e.g. an employer monthly schedule / investigation extract). It is intentionally
//! distinct from the department `gov-dept-ird` DB schema — the [`crate::transform`]
//! layer is responsible for mapping one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIrdBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawIrdCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIrdCitizen {
    pub ird_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    pub income: Option<RawIncome>,
    pub tax_assessment: Option<RawTaxAssessment>,
    pub gst: RawGst,
    pub kiwisaver: Option<RawKiwiSaver>,
    pub working_for_families: Option<RawWorkingForFamilies>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIncome {
    pub assessment_year: i32,
    #[serde(default)]
    pub employment_income: f64,
    #[serde(default)]
    pub self_employment_income: f64,
    #[serde(default)]
    pub rental_income: f64,
    #[serde(default)]
    pub other_income: f64,
    #[serde(default)]
    pub total_deductions: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTaxAssessment {
    pub assessment_year: i32,
    #[serde(default = "default_tax_code")]
    pub tax_code: String,
    pub total_income: f64,
    pub taxable_income: f64,
    pub tax_liability: f64,
    pub tax_paid: f64,
    #[serde(default)]
    pub tax_refund_due: f64,
    #[serde(default)]
    pub tax_owing: f64,
    #[serde(default = "default_assessment_status")]
    pub assessment_status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGst {
    pub registered: bool,
    #[serde(default)]
    pub gst_number: Option<String>,
    #[serde(default)]
    pub filing_frequency: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawKiwiSaver {
    pub membership_status: String,
    pub contribution_rate: f64,
    #[serde(default)]
    pub employer_contribution_rate: Option<f64>,
    #[serde(default)]
    pub scheme: Option<String>,
    #[serde(default)]
    pub total_balance: Option<f64>,
    #[serde(default)]
    pub government_contribution_eligible: bool,
    #[serde(default)]
    pub first_home_buyer_eligible: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWorkingForFamilies {
    pub eligible: bool,
    #[serde(default)]
    pub number_of_dependant_children: i32,
    #[serde(default)]
    pub income_threshold: f64,
    #[serde(default)]
    pub family_tax_credit: Option<f64>,
    #[serde(default)]
    pub in_work_tax_credit: Option<f64>,
    #[serde(default)]
    pub best_start_payment: Option<f64>,
    #[serde(default)]
    pub minimum_family_tax_credit: Option<f64>,
    #[serde(default)]
    pub payment_frequency: Option<String>,
}

fn default_tax_code() -> String {
    "M".to_owned()
}

fn default_assessment_status() -> String {
    "final".to_owned()
}
