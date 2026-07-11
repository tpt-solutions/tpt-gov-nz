//! Raw MSD legacy data format.
//!
//! Mirrors the shape of a batch extract from the MSD / StudyLink / Work and Income
//! legacy systems. Distinct from the department `gov-dept-msd` DB schema; the
//! [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMsdBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMsdCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMsdCitizen {
    pub client_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub studylink: Option<RawStudyLink>,
    #[serde(default)]
    pub case_history: Vec<RawCaseEvent>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawStudyLink {
    pub has_student_loan: bool,
    #[serde(default)]
    pub loan_balance: Option<f64>,
    #[serde(default)]
    pub repayment_plan: Option<String>,
    pub has_allowance: bool,
    #[serde(default)]
    pub allowance_type: Option<String>,
    #[serde(default)]
    pub next_payment_date: Option<String>,
    #[serde(default)]
    pub weekly_amount: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCaseEvent {
    pub event_id: String,
    pub event_date: String,
    pub service_line: String,
    pub summary: String,
}
