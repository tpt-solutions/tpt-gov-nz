//! Raw Ministry of Education legacy data format.
//!
//! Mirrors the shape of a batch extract from the Ministry of Education legacy systems.
//! Distinct from the department `gov-dept-moe` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMoeBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMoeCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMoeCitizen {
    pub moe_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enrolment: Option<RawEnrolment>,
    #[serde(default)]
    pub student_support: Vec<RawStudentSupport>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEnrolment {
    pub school: String,
    pub year_level: i32,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawStudentSupport {
    pub service: String,
    pub status: String,
    pub next_review: String,
}
