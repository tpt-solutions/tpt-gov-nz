//! Raw Corrections legacy data format.
//!
//! Mirrors the shape of a batch extract from the Department of Corrections legacy
//! systems. Distinct from the department `gov-dept-corrections` DB schema; the
//! [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCorrectionsBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawCorrectionsCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCorrectionsCitizen {
    pub corrections_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub probation: Option<RawProbation>,
    #[serde(default)]
    pub case: Vec<RawCase>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProbation {
    pub status: String,
    pub officer_name: String,
    pub next_report_date: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCase {
    pub case_number: String,
    pub sentence_type: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub summary: String,
}
