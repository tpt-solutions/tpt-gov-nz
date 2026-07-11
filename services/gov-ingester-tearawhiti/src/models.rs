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
    pub tearawhiti_id: String,
}

#[derive(Debug, Clone)]
pub struct TreatySettlementsEntity {
    pub iwi: String,
    pub status: String,
    pub settled_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct EngagementsEntity {
    pub topic: String,
    pub engagement_date: chrono::NaiveDate,
    pub outcome: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub treaty_settlements: Vec<TreatySettlementsEntity>,
    pub engagements: Vec<EngagementsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
