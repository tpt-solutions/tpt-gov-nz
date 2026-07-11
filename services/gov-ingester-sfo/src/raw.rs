//! Raw Serious Fraud Office legacy data format.
//!
//! Mirrors the shape of a batch extract from the Serious Fraud Office legacy systems.
//! Distinct from the department `gov-dept-sfo` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSfoBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawSfoCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSfoCitizen {
    pub sfo_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub investigations: Vec<RawInvestigations>,
    #[serde(default)]
    pub outcomes: Vec<RawOutcomes>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawInvestigations {
    pub reference: String,
    pub matter: String,
    pub status: String,
    pub opened_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawOutcomes {
    pub reference: String,
    pub result: String,
    pub result_date: String,
}
