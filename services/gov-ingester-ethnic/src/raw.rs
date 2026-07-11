//! Raw Ministry for Ethnic Communities legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for Ethnic Communities legacy systems.
//! Distinct from the department `gov-dept-ethnic` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEthnicBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawEthnicCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEthnicCitizen {
    pub ethnic_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub programmes: Vec<RawProgrammes>,
    #[serde(default)]
    pub community_grants: Vec<RawCommunityGrants>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProgrammes {
    pub programme_name: String,
    pub status: String,
    pub year: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCommunityGrants {
    pub grant_name: String,
    pub amount: f64,
    pub status: String,
}
