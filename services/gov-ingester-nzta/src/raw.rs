//! Raw NZTA legacy data format.
//!
//! Mirrors the shape of a batch extract from the NZTA legacy systems. Distinct from
//! the department `gov-dept-nzta` DB schema; the [`crate::transform`] layer maps one
//! to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNztaBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawNztaCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawNztaCitizen {
    pub driver_licence_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub driver_licence: Option<RawDriverLicence>,
    #[serde(default)]
    pub vehicles: Vec<RawVehicle>,
    #[serde(default)]
    pub ruc: Vec<RawRuc>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDriverLicence {
    pub full_name: String,
    pub licence_class: String,
    pub expiry_date: String,
    #[serde(default)]
    pub conditions: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawVehicle {
    pub registration: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub fuel_type: String,
    pub registration_expiry: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRuc {
    pub vehicle_rego: String,
    pub licence_type: String,
    pub expiry_date: String,
    pub units_remaining: i32,
}
