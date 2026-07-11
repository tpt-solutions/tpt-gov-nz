//! Raw LINZ legacy data format.
//!
//! Mirrors the shape of a batch extract from the LINZ legacy systems.
//! Distinct from the department `gov-dept-linz` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLinzBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawLinzCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawLinzCitizen {
    pub customer_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub titles: Vec<RawTitle>,
    #[serde(default)]
    pub ownership: Vec<RawOwnership>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTitle {
    pub title_number: String,
    pub property_address: String,
    pub land_area_sqm: f64,
    pub estate_type: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawOwnership {
    pub title_number: String,
    pub ownership_share: String,
    pub registered_owners: Vec<String>,
}
