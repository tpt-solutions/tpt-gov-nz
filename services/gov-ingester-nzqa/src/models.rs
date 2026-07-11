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
    pub nsn: String,
}

#[derive(Debug, Clone)]
pub struct QualificationEntity {
    pub qualification_id: String,
    pub title: String,
    pub level: i32,
    pub awarded_date: chrono::NaiveDate,
    pub provider: String,
}

#[derive(Debug, Clone)]
pub struct TranscriptEntity {
    pub record_summary: Option<String>,
    pub total_credits: Option<i32>,
    pub credit_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub qualifications: Vec<QualificationEntity>,
    pub transcript: Option<TranscriptEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
