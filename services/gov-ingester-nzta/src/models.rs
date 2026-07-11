use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionSummary {
    pub source: String,
    pub batch_id: Option<String>,
    pub citizens_processed: u32,
    pub rows_inserted: u32,
    pub rows_updated: u32,
    pub status: IngestionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IngestionStatus {
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct CitizenEntity {
    pub did: String,
    pub driver_licence_number: String,
}

#[derive(Debug, Clone)]
pub struct DriverLicenceEntity {
    pub licence_number: String,
    pub full_name: String,
    pub licence_class: String,
    pub expiry_date: chrono::NaiveDate,
    pub conditions: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VehicleEntity {
    pub registration: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub fuel_type: String,
    pub registration_expiry: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct RucEntity {
    pub vehicle_rego: String,
    pub licence_type: String,
    pub expiry_date: chrono::NaiveDate,
    pub units_remaining: i32,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub driver_licence: Option<DriverLicenceEntity>,
    pub vehicles: Vec<VehicleEntity>,
    pub ruc: Vec<RucEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
