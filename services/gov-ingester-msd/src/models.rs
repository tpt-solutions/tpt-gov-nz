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
pub struct StudyLinkEntity {
    pub has_student_loan: bool,
    pub loan_balance: Option<f64>,
    pub repayment_plan: Option<String>,
    pub has_allowance: bool,
    pub allowance_type: Option<String>,
    pub next_payment_date: Option<chrono::NaiveDate>,
    pub weekly_amount: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CaseEventEntity {
    pub event_id: String,
    pub event_date: chrono::NaiveDate,
    pub service_line: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub studylink: Option<StudyLinkEntity>,
    pub case_events: Vec<CaseEventEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
