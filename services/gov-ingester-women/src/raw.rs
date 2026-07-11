//! Raw Ministry for Women legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for Women legacy systems.
//! Distinct from the department `gov-dept-women` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWomenBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawWomenCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawWomenCitizen {
    pub women_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub programmes: Vec<RawProgrammes>,
    #[serde(default)]
    pub insights: Vec<RawInsights>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProgrammes {
    pub programme_name: String,
    pub status: String,
    pub year: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawInsights {
    pub topic: String,
    pub summary: String,
    pub published: String,
}
