//! Raw Department of the Prime Minister and Cabinet legacy data format.
//!
//! Mirrors the shape of a batch extract from the Department of the Prime Minister and Cabinet legacy systems.
//! Distinct from the department `gov-dept-dpmc` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDpmcBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawDpmcCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDpmcCitizen {
    pub dpmc_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub honours: Vec<RawHonours>,
    #[serde(default)]
    pub engagements: Vec<RawEngagements>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawHonours {
    pub award_year: i32,
    pub award: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEngagements {
    pub event_name: String,
    pub event_date: String,
    pub location: String,
}
