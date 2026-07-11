//! Raw Ministry of Defence legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry of Defence legacy systems.
//! Distinct from the department `gov-dept-defence` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDefenceBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawDefenceCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDefenceCitizen {
    pub defence_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub procurements: Vec<RawProcurements>,
    #[serde(default)]
    pub bases: Vec<RawBases>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProcurements {
    pub programme: String,
    pub value: f64,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawBases {
    pub name: String,
    pub location: String,
    pub status: String,
}
