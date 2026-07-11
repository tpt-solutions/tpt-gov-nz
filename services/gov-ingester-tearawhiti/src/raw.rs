//! Raw Te Arawhiti legacy data format.
//!
//! Mirrors the shape of a batch extract from the Te Arawhiti legacy systems.
//! Distinct from the department `gov-dept-tearawhiti` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTearawhitiBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawTearawhitiCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTearawhitiCitizen {
    pub tearawhiti_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub treaty_settlements: Vec<RawTreatySettlements>,
    #[serde(default)]
    pub engagements: Vec<RawEngagements>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTreatySettlements {
    pub iwi: String,
    pub status: String,
    pub settled_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEngagements {
    pub topic: String,
    pub engagement_date: String,
    pub outcome: String,
}
