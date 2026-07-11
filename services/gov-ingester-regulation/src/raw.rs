//! Raw Ministry for Regulation legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for Regulation legacy systems.
//! Distinct from the department `gov-dept-regulation` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRegulationBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawRegulationCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRegulationCitizen {
    pub regulation_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub regulatory_reviews: Vec<RawRegulatoryReviews>,
    #[serde(default)]
    pub proposals: Vec<RawProposals>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRegulatoryReviews {
    pub topic: String,
    pub agency: String,
    pub status: String,
    pub review_year: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProposals {
    pub title: String,
    pub status: String,
}
