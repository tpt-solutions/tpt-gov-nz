//! Raw New Zealand Defence Force legacy data format.
//!
//! Mirrors the shape of a batch extract from the New Zealand Defence Force legacy systems.
//! Distinct from the department `gov-dept-nzdf` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzdfBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawNzdfCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzdfCitizen {
    pub nzdf_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub service_records: Vec<RawServiceRecords>,
    #[serde(default)]
    pub deployments: Vec<RawDeployments>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawServiceRecords {
    pub service_no: String,
    pub branch: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDeployments {
    pub operation: String,
    pub country: String,
    pub year: i32,
}
