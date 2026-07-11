//! Raw HUD legacy data format.
//!
//! Mirrors the shape of a batch extract from the HUD / Kainga Ora legacy systems.
//! Distinct from the department `gov-dept-hud` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawHudBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawHudCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawHudCitizen {
    pub client_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub applications: Vec<RawApplication>,
    #[serde(default)]
    pub tenancies: Vec<RawTenancy>,
    #[serde(default)]
    pub maintenance_requests: Vec<RawMaintenanceRequest>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawApplication {
    pub application_number: String,
    pub application_type: String,
    pub status: String,
    #[serde(default)]
    pub priority_band: Option<String>,
    #[serde(default)]
    pub bedrooms_needed: Option<i32>,
    pub submitted_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTenancy {
    pub tenancy_id: String,
    pub property_address: String,
    pub weekly_rent: f64,
    pub income_related_rent: bool,
    pub start_date: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMaintenanceRequest {
    pub request_number: String,
    pub category: String,
    pub status: String,
    pub description: String,
    pub requested_date: String,
}
