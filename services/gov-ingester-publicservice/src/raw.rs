//! Raw Te Kawa Mataaho Public Service Commission legacy data format.
//!
//! Mirrors the shape of a batch extract from the Te Kawa Mataaho Public Service Commission legacy systems.
//! Distinct from the department `gov-dept-publicservice` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPublicserviceBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawPublicserviceCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPublicserviceCitizen {
    pub psc_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub workforce: Vec<RawWorkforce>,
    #[serde(default)]
    pub agency_ratings: Vec<RawAgencyRatings>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWorkforce {
    pub report_year: i32,
    pub agency: String,
    pub headcount: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawAgencyRatings {
    pub agency: String,
    pub rating: String,
    pub rating_year: i32,
}
