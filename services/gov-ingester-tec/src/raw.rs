//! Raw Tertiary Education Commission legacy data format.
//!
//! Mirrors the shape of a batch extract from the Tertiary Education Commission legacy systems.
//! Distinct from the department `gov-dept-tec` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTecBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawTecCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTecCitizen {
    pub tec_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub funding: Vec<RawFunding>,
    #[serde(default)]
    pub courses: Vec<RawCourses>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawFunding {
    pub provider: String,
    pub amount: f64,
    pub year: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCourses {
    pub course_name: String,
    pub provider: String,
    pub status: String,
}
