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
    pub fenz_id: String,
}

#[derive(Debug, Clone)]
pub struct FireSafetyEntity {
    pub property: String,
    pub grade: String,
    pub last_inspection: chrono::NaiveDate,
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
    pub fire_safety: Option<FireSafetyEntity>,
    pub incidents: Vec<IncidentsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
