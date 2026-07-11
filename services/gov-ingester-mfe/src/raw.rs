//! Raw Ministry for the Environment legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for the Environment legacy systems.
//! Distinct from the department `gov-dept-mfe` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMfeBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMfeCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMfeCitizen {
    pub mfe_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub emissions: Vec<RawEmissions>,
    #[serde(default)]
    pub reports: Vec<RawReports>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEmissions {
    pub report_year: i32,
    pub sector: String,
    pub tonnes_co2e: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawReports {
    pub title: String,
    pub published: String,
    pub status: String,
}
