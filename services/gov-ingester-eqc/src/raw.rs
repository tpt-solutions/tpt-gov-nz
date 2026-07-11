//! Raw Earthquake Commission (Toka Tū Ake) legacy data format.
//!
//! Mirrors the shape of a batch extract from the Earthquake Commission (Toka Tū Ake) legacy systems.
//! Distinct from the department `gov-dept-eqc` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEqcBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawEqcCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEqcCitizen {
    pub eqc_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub claims: Vec<RawClaims>,
    #[serde(default)]
    pub cover: Option<RawCover>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawClaims {
    pub reference: String,
    pub property: String,
    pub status: String,
    pub lodged_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCover {
    pub property: String,
    pub sum_insured: f64,
    pub valid_to: String,
}
