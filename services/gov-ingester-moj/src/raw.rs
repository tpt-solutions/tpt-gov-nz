//! Raw MOJ legacy data format.
//!
//! Mirrors the shape of a batch extract from the MOJ legacy systems. Distinct from
//! the department `gov-dept-moj` DB schema; the [`crate::transform`] layer maps
//! one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMojBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMojCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMojCitizen {
    pub client_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub fines: Vec<RawFine>,
    #[serde(default)]
    pub disputes: Vec<RawDispute>,
    #[serde(default)]
    pub court_records: Vec<RawCourtRecord>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFine {
    pub fine_number: String,
    pub fine_type: String,
    pub status: String,
    pub amount: f64,
    pub offense_date: String,
    pub due_date: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDispute {
    pub dispute_number: String,
    pub claim_type: String,
    pub status: String,
    #[serde(default)]
    pub amount_claimed: Option<f64>,
    #[serde(default)]
    pub hearing_date: Option<String>,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCourtRecord {
    pub case_number: String,
    pub case_type: String,
    pub status: String,
    #[serde(default)]
    pub next_hearing_date: Option<String>,
    pub description: String,
}
