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
    pub mch_id: String,
}

#[derive(Debug, Clone)]
pub struct HeritageSitesEntity {
    pub name: String,
    pub status: String,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct GrantsEntity {
    pub grant_name: String,
    pub amount: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub heritage_sites: Vec<HeritageSitesEntity>,
    pub grants: Vec<GrantsEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
