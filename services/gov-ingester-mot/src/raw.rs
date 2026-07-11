//! Raw Ministry of Transport legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry of Transport legacy systems.
//! Distinct from the department `gov-dept-mot` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMotBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMotCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMotCitizen {
    pub mot_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub strategies: Vec<RawStrategies>,
    #[serde(default)]
    pub programmes: Vec<RawProgrammes>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawStrategies {
    pub title: String,
    pub year: i32,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawProgrammes {
    pub name: String,
    pub budget: f64,
    pub status: String,
}
