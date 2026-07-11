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
pub struct FineEntity {
    pub fine_number: String,
    pub fine_type: String,
    pub status: String,
    pub amount: f64,
    pub offense_date: chrono::NaiveDate,
    pub due_date: chrono::NaiveDate,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DisputeEntity {
    pub dispute_number: String,
    pub claim_type: String,
    pub status: String,
    pub amount_claimed: Option<f64>,
    pub hearing_date: Option<chrono::NaiveDate>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CourtRecordEntity {
    pub case_number: String,
    pub case_type: String,
    pub status: String,
    pub next_hearing_date: Option<chrono::NaiveDate>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub fines: Vec<FineEntity>,
    pub disputes: Vec<DisputeEntity>,
    pub court_records: Vec<CourtRecordEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
