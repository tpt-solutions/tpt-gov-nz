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
    pub psc_id: String,
}

#[derive(Debug, Clone)]
pub struct WorkforceEntity {
    pub report_year: i32,
    pub agency: String,
    pub headcount: i32,
}

#[derive(Debug, Clone)]
pub struct AgencyRatingsEntity {
    pub agency: String,
    pub rating: String,
    pub rating_year: i32,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub workforce: Vec<WorkforceEntity>,
    pub agency_ratings: Vec<AgencyRatingsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
