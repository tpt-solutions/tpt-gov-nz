//! Raw Retirement Commission (Te Ara Ahunga Ora) legacy data format.
//!
//! Mirrors the shape of a batch extract from the Retirement Commission (Te Ara Ahunga Ora) legacy systems.
//! Distinct from the department `gov-dept-retirement` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRetirementBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawRetirementCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRetirementCitizen {
    pub retirement_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub retirement_plan: Option<RawRetirementPlan>,
    #[serde(default)]
    pub guidance: Vec<RawGuidance>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRetirementPlan {
    pub has_plan: bool,
    pub retirement_age: i32,
    pub last_review: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGuidance {
    pub topic: String,
    pub summary: String,
    pub published: String,
}
