//! Raw Statistics New Zealand legacy data format.
//!
//! Mirrors the shape of a batch extract from the Stats NZ legacy systems.
//! Distinct from the department `gov-dept-statsnz` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawStatsnzBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawStatsnzCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawStatsnzCitizen {
    pub stats_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub census: Vec<RawCensus>,
    #[serde(default)]
    pub profile: Option<RawProfile>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCensus {
    pub census_year: i32,
    pub dwelling_type: String,
    pub household_size: i32,
    pub region: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProfile {
    pub data_summary: String,
    pub record_count: i32,
    pub last_updated: String,
}
