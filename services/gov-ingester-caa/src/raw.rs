//! Raw Civil Aviation Authority legacy data format.
//!
//! Mirrors the shape of a batch extract from the Civil Aviation Authority legacy systems.
//! Distinct from the department `gov-dept-caa` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCaaBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawCaaCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCaaCitizen {
    pub caa_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub licences: Vec<RawLicences>,
    #[serde(default)]
    pub aircraft: Vec<RawAircraft>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLicences {
    pub licence_no: String,
    pub category: String,
    pub status: String,
    pub expires: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawAircraft {
    pub registration: String,
    pub aircraft_type: String,
    pub status: String,
}
