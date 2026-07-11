//! Raw Customs legacy data format.
//!
//! Mirrors the shape of a batch extract from the New Zealand Customs Service legacy
//! systems. Distinct from the department `gov-dept-customs` DB schema; the
//! [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCustomsBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawCustomsCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCustomsCitizen {
    pub traveller_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub travel: Option<RawTravel>,
    #[serde(default)]
    pub declarations: Vec<RawDeclaration>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTravel {
    pub passport_number: String,
    pub last_arrival: String,
    pub arrival_port: String,
    pub frequent_traveller: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDeclaration {
    pub declaration_id: String,
    pub date: String,
    pub country_from: String,
    pub goods_declared: String,
    pub status: String,
}
