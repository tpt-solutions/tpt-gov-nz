//! Raw Government Communications Security Bureau legacy data format.
//!
//! Mirrors the shape of a batch extract from the Government Communications Security Bureau legacy systems.
//! Distinct from the department `gov-dept-gcsb` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGcsbBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawGcsbCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGcsbCitizen {
    pub gcsb_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mandates: Vec<RawMandates>,
    #[serde(default)]
    pub engagements: Vec<RawEngagements>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMandates {
    pub reference: String,
    pub agency: String,
    pub status: String,
    pub issued_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEngagements {
    pub partner: String,
    pub engagement_type: String,
    pub engagement_date: String,
}
