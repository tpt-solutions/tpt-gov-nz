//! Raw Education Review Office legacy data format.
//!
//! Mirrors the shape of a batch extract from the Education Review Office legacy systems.
//! Distinct from the department `gov-dept-ero` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEroBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawEroCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEroCitizen {
    pub ero_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub reviews: Vec<RawReviews>,
    #[serde(default)]
    pub reports: Vec<RawReports>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawReviews {
    pub school: String,
    pub rating: String,
    pub review_date: String,
    pub next_review: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawReports {
    pub title: String,
    pub published: String,
}
