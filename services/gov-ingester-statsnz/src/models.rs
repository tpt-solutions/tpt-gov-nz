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
    pub stats_id: String,
}

#[derive(Debug, Clone)]
pub struct CensusEntity {
    pub census_year: i32,
    pub dwelling_type: String,
    pub household_size: i32,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct ProfileEntity {
    pub data_summary: String,
    pub record_count: i32,
    pub last_updated: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub census: Vec<CensusEntity>,
    pub profile: Option<ProfileEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
