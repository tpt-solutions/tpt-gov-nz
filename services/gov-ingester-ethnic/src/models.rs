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
    pub ethnic_id: String,
}

#[derive(Debug, Clone)]
pub struct ProgrammesEntity {
    pub programme_name: String,
    pub status: String,
    pub year: i32,
}

#[derive(Debug, Clone)]
pub struct CommunityGrantsEntity {
    pub grant_name: String,
    pub amount: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub programmes: Vec<ProgrammesEntity>,
    pub community_grants: Vec<CommunityGrantsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
