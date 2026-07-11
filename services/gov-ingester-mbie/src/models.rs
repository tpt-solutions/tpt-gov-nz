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
    pub person_id: String,
}

#[derive(Debug, Clone)]
pub struct BusinessEntity {
    pub nzbn: String,
    pub entity_name: String,
    pub entity_type: String,
    pub status: String,
    pub registered_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct DirectorshipEntity {
    pub nzbn: String,
    pub entity_name: String,
    pub role: String,
    pub appointed_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub businesses: Vec<BusinessEntity>,
    pub directorships: Vec<DirectorshipEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
