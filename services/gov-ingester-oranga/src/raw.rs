//! Raw Oranga Tamariki legacy data format.
//!
//! Mirrors the shape of a batch extract from the Oranga Tamariki legacy systems.
//! Distinct from the department `gov-dept-oranga` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawOrangaBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawOrangaCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawOrangaCitizen {
    pub oranga_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub care_placements: Vec<RawCarePlacements>,
    #[serde(default)]
    pub support_services: Vec<RawSupportServices>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCarePlacements {
    pub placement_type: String,
    pub start_date: String,
    pub region: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSupportServices {
    pub service: String,
    pub status: String,
    pub next_review: String,
}
