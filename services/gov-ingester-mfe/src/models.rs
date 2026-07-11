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
    pub mfe_id: String,
}

#[derive(Debug, Clone)]
pub struct EmissionsEntity {
    pub report_year: i32,
    pub sector: String,
    pub tonnes_co2e: f64,
}

#[derive(Debug, Clone)]
pub struct ReportsEntity {
    pub title: String,
    pub published: chrono::NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub emissions: Vec<EmissionsEntity>,
    pub reports: Vec<ReportsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
