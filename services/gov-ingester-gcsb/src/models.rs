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
    pub gcsb_id: String,
}

#[derive(Debug, Clone)]
pub struct MandatesEntity {
    pub reference: String,
    pub agency: String,
    pub status: String,
    pub issued_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct EngagementsEntity {
    pub partner: String,
    pub engagement_type: String,
    pub engagement_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub mandates: Vec<MandatesEntity>,
    pub engagements: Vec<EngagementsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
