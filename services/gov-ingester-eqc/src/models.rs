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
    pub eqc_id: String,
}

#[derive(Debug, Clone)]
pub struct ClaimsEntity {
    pub reference: String,
    pub property: String,
    pub status: String,
    pub lodged_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct CoverEntity {
    pub property: String,
    pub sum_insured: f64,
    pub valid_to: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub claims: Vec<ClaimsEntity>,
    pub cover: Option<CoverEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
