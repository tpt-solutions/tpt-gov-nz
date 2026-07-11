//! Raw New Zealand Security Intelligence Service legacy data format.
//!
//! Mirrors the shape of a batch extract from the New Zealand Security Intelligence Service legacy systems.
//! Distinct from the department `gov-dept-nzsis` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzsisBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawNzsisCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzsisCitizen {
    pub nzsis_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mandates: Vec<RawMandates>,
    #[serde(default)]
    pub threats: Vec<RawThreats>,
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
pub struct RawThreats {
    pub reference: String,
    pub category: String,
    pub status: String,
    pub assessed_date: String,
}
