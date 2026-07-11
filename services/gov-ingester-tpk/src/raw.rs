//! Raw TPK legacy data format.
//!
//! Mirrors the shape of a batch extract from the Te Puni Kōkiri legacy systems.
//! Distinct from the department `gov-dept-tpk` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTpkBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawTpkCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTpkCitizen {
    pub tpk_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub programmes: Vec<RawProgramme>,
    #[serde(default)]
    pub funding: Vec<RawFunding>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProgramme {
    pub programme_name: String,
    pub status: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFunding {
    pub grant_id: String,
    pub amount: i64,
    pub purpose: String,
    pub status: String,
}
