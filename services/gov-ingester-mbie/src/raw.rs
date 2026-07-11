//! Raw MBIE legacy data format.
//!
//! Mirrors the shape of a batch extract from the MBIE legacy systems (Companies
//! Office / NZBN register). Distinct from the department `gov-dept-mbie` DB schema;
//! the [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMbieBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMbieCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMbieCitizen {
    pub person_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub business_registrations: Vec<RawBusiness>,
    #[serde(default)]
    pub directorships: Vec<RawDirectorship>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawBusiness {
    pub nzbn: String,
    pub entity_name: String,
    pub entity_type: String,
    pub status: String,
    pub registered_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDirectorship {
    pub nzbn: String,
    pub entity_name: String,
    pub role: String,
    pub appointed_date: String,
}
