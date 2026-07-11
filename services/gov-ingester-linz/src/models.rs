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
    pub customer_id: String,
}

#[derive(Debug, Clone)]
pub struct TitleEntity {
    pub title_number: String,
    pub property_address: String,
    pub land_area_sqm: f64,
    pub estate_type: String,
}

#[derive(Debug, Clone)]
pub struct OwnershipEntity {
    pub title_number: String,
    pub ownership_share: String,
    pub registered_owners: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub titles: Vec<TitleEntity>,
    pub ownership: Vec<OwnershipEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
