//! Raw WINZ legacy data format.
//!
//! This mirrors the shape of a batch file as produced by the WINZ legacy systems.
//! It is intentionally distinct from the department `gov-dept-winz` DB schema —
//! the [`crate::transform`] layer is responsible for mapping one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWinzBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawWinzCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWinzCitizen {
    pub client_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub benefits: Vec<RawBenefit>,
    #[serde(default)]
    pub payments: Vec<RawPayment>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawBenefit {
    pub benefit_type: String,
    pub weekly_amount: f64,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub review_date: Option<String>,
    #[serde(default = "default_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPayment {
    pub benefit_type: String,
    pub payment_date: String,
    pub amount: f64,
    #[serde(default = "default_method")]
    pub method: String,
}

fn default_status() -> String {
    "active".to_owned()
}

fn default_method() -> String {
    "bank-deposit".to_owned()
}
