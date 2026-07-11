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
    pub nzdf_id: String,
}

#[derive(Debug, Clone)]
pub struct ServiceRecordsEntity {
    pub service_no: String,
    pub branch: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct DeploymentsEntity {
    pub operation: String,
    pub country: String,
    pub year: i32,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub service_records: Vec<ServiceRecordsEntity>,
    pub deployments: Vec<DeploymentsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
