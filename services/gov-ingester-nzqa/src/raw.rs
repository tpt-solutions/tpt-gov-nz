//! Raw NZQA legacy data format.
//!
//! Mirrors the shape of a batch extract from the NZQA / Ministry of Education legacy systems.
//! Distinct from the department `gov-dept-nzqa` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzqaBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawNzqaCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNzqaCitizen {
    pub nsn: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub qualifications: Vec<RawQualification>,
    #[serde(default)]
    pub transcript: Option<RawTranscript>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawQualification {
    pub qualification_id: String,
    pub title: String,
    pub level: i32,
    pub awarded_date: String,
    pub provider: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTranscript {
    #[serde(default)]
    pub record_summary: Option<String>,
    #[serde(default)]
    pub total_credits: Option<i32>,
    #[serde(default)]
    pub credit_summary: Option<String>,
}
