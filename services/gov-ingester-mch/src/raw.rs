//! Raw Ministry for Culture and Heritage legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for Culture and Heritage legacy systems.
//! Distinct from the department `gov-dept-mch` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMchBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMchCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMchCitizen {
    pub mch_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub heritage_sites: Vec<RawHeritageSites>,
    #[serde(default)]
    pub grants: Vec<RawGrants>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawHeritageSites {
    pub name: String,
    pub status: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGrants {
    pub grant_name: String,
    pub amount: f64,
    pub status: String,
}
