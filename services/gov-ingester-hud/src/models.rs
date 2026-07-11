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
    pub client_number: String,
}

#[derive(Debug, Clone)]
pub struct ApplicationEntity {
    pub application_number: String,
    pub application_type: String,
    pub status: String,
    pub priority_band: Option<String>,
    pub bedrooms_needed: Option<i32>,
    pub submitted_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TenancyEntity {
    pub tenancy_id: String,
    pub property_address: String,
    pub weekly_rent: f64,
    pub income_related_rent: bool,
    pub start_date: chrono::NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct MaintenanceRequestEntity {
    pub request_number: String,
    pub category: String,
    pub status: String,
    pub description: String,
    pub requested_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub applications: Vec<ApplicationEntity>,
    pub tenancies: Vec<TenancyEntity>,
    pub maintenance_requests: Vec<MaintenanceRequestEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
