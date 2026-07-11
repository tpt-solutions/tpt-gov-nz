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
    pub maritime_id: String,
}

#[derive(Debug, Clone)]
pub struct VesselsEntity {
    pub vessel_name: String,
    pub flag: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct IncidentsEntity {
    pub reference: String,
    pub incident_type: String,
    pub incident_date: chrono::NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub vessels: Vec<VesselsEntity>,
    pub incidents: Vec<IncidentsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
