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
    pub doc_id: String,
}

#[derive(Debug, Clone)]
pub struct PermitEntity {
    pub permit_number: String,
    pub activity: String,
    pub location: String,
    pub status: String,
    pub expires_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct ConcessionEntity {
    pub concession_id: String,
    pub r#type: String,
    pub holder: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub permits: Vec<PermitEntity>,
    pub concessions: Vec<ConcessionEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
