//! Raw Maritime New Zealand legacy data format.
//!
//! Mirrors the shape of a batch extract from the Maritime New Zealand legacy systems.
//! Distinct from the department `gov-dept-maritime` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMaritimeBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMaritimeCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMaritimeCitizen {
    pub maritime_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub vessels: Vec<RawVessels>,
    #[serde(default)]
    pub incidents: Vec<RawIncidents>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawVessels {
    pub vessel_name: String,
    pub flag: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIncidents {
    pub reference: String,
    pub incident_type: String,
    pub incident_date: String,
    pub status: String,
}
