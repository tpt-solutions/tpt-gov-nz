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
    pub tpk_id: String,
}

#[derive(Debug, Clone)]
pub struct ProgrammeEntity {
    pub programme_name: String,
    pub status: String,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct FundingEntity {
    pub grant_id: String,
    pub amount: i64,
    pub purpose: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub programmes: Vec<ProgrammeEntity>,
    pub funding: Vec<FundingEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
