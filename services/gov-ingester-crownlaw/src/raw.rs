//! Raw Crown Law Office legacy data format.
//!
//! Mirrors the shape of a batch extract from the Crown Law Office legacy systems.
//! Distinct from the department `gov-dept-crownlaw` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCrownlawBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawCrownlawCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCrownlawCitizen {
    pub crownlaw_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub legal_opinions: Vec<RawLegalOpinions>,
    #[serde(default)]
    pub litigation: Vec<RawLitigation>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLegalOpinions {
    pub reference: String,
    pub topic: String,
    pub issued_date: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLitigation {
    pub case_name: String,
    pub crown_role: String,
    pub status: String,
}
