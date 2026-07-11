//! Raw Fire and Emergency New Zealand legacy data format.
//!
//! Mirrors the shape of a batch extract from the Fire and Emergency New Zealand legacy systems.
//! Distinct from the department `gov-dept-fenz` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFenzBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawFenzCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFenzCitizen {
    pub fenz_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub fire_safety: Option<RawFireSafety>,
    #[serde(default)]
    pub incidents: Vec<RawIncidents>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFireSafety {
    pub property: String,
    pub grade: String,
    pub last_inspection: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawIncidents {
    pub reference: String,
    pub incident_type: String,
    pub incident_date: String,
    pub status: String,
}
