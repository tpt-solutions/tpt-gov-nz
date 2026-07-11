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
    pub corrections_id: String,
}

#[derive(Debug, Clone)]
pub struct ProbationEntity {
    pub status: String,
    pub officer_name: String,
    pub next_report_date: chrono::NaiveDate,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct CaseEntity {
    pub case_number: String,
    pub sentence_type: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub probation: Option<ProbationEntity>,
    pub cases: Vec<CaseEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
