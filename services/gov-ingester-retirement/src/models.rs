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
    pub retirement_id: String,
}

#[derive(Debug, Clone)]
pub struct RetirementPlanEntity {
    pub has_plan: bool,
    pub retirement_age: i32,
    pub last_review: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct GuidanceEntity {
    pub topic: String,
    pub summary: String,
    pub published: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub retirement_plan: Option<RetirementPlanEntity>,
    pub guidance: Vec<GuidanceEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
