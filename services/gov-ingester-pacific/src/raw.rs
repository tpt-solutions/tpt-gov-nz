//! Raw Ministry for Pacific Peoples legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry for Pacific Peoples legacy systems.
//! Distinct from the department `gov-dept-pacific` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPacificBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawPacificCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPacificCitizen {
    pub pacific_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub programmes: Vec<RawProgrammes>,
    #[serde(default)]
    pub language_services: Vec<RawLanguageServices>,
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
pub struct RawLanguageServices {
    pub service: String,
    pub region: String,
    pub status: String,
}
