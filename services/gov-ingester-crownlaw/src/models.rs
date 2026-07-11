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
    pub crownlaw_id: String,
}

#[derive(Debug, Clone)]
pub struct LegalOpinionsEntity {
    pub reference: String,
    pub topic: String,
    pub issued_date: chrono::NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct LitigationEntity {
    pub case_name: String,
    pub crown_role: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub legal_opinions: Vec<LegalOpinionsEntity>,
    pub litigation: Vec<LitigationEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
