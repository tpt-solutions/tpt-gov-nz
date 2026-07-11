//! Raw Ministry of Foreign Affairs and Trade legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry of Foreign Affairs and Trade legacy systems.
//! Distinct from the department `gov-dept-mfat` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMfatBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMfatCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMfatCitizen {
    pub mfat_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub overseas_missions: Vec<RawOverseasMissions>,
    #[serde(default)]
    pub travel_advisories: Vec<RawTravelAdvisories>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawOverseasMissions {
    pub country: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTravelAdvisories {
    pub country: String,
    pub level: String,
    pub updated: String,
}
