//! Raw WorkSafe New Zealand legacy data format.
//!
//! Mirrors the shape of a batch extract from the WorkSafe New Zealand legacy systems.
//! Distinct from the department `gov-dept-worksafe` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWorksafeBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawWorksafeCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWorksafeCitizen {
    pub worksafe_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub inspections: Vec<RawInspections>,
    #[serde(default)]
    pub investigations: Vec<RawInvestigations>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawInspections {
    pub reference: String,
    pub site: String,
    pub inspection_date: String,
    pub outcome: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawInvestigations {
    pub reference: String,
    pub matter: String,
    pub status: String,
    pub opened_date: String,
}
