use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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
    pub oranga_id: String,
}

#[derive(Debug, Clone)]
pub struct CarePlacementsEntity {
    pub placement_type: String,
    pub start_date: chrono::NaiveDate,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct SupportServicesEntity {
    pub service: String,
    pub status: String,
    pub next_review: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub care_placements: Vec<CarePlacementsEntity>,
    pub support_services: Vec<SupportServicesEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
