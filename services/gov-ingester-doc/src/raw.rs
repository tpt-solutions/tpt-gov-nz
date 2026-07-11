//! Raw DOC legacy data format.
//!
//! Mirrors the shape of a batch extract from the Department of Conservation legacy
//! systems. Distinct from the department `gov-dept-doc` DB schema; the
//! [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDocBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawDocCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDocCitizen {
    pub doc_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub permits: Vec<RawPermit>,
    #[serde(default)]
    pub concessions: Vec<RawConcession>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPermit {
    pub permit_number: String,
    pub activity: String,
    pub location: String,
    pub status: String,
    pub expires_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawConcession {
    pub concession_id: String,
    pub r#type: String,
    pub holder: String,
    pub start_date: String,
    pub end_date: String,
}
