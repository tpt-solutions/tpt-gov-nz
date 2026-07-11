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
pub struct InfringementEntity {
    pub ticket_number: String,
    pub offense_type: String,
    pub status: String,
    pub amount: f64,
    pub issue_date: chrono::NaiveDate,
    pub location: Option<String>,
    pub demerit_points: Option<i32>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ReportEntity {
    pub report_number: String,
    pub report_type: String,
    pub status: String,
    pub filed_date: chrono::NaiveDate,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub infringements: Vec<InfringementEntity>,
    pub reports: Vec<ReportEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
