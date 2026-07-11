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
    pub traveller_id: String,
}

#[derive(Debug, Clone)]
pub struct TravelEntity {
    pub passport_number: String,
    pub last_arrival: chrono::NaiveDate,
    pub arrival_port: String,
    pub frequent_traveller: bool,
}

#[derive(Debug, Clone)]
pub struct DeclarationEntity {
    pub declaration_id: String,
    pub date: chrono::NaiveDate,
    pub country_from: String,
    pub goods_declared: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub travel: Option<TravelEntity>,
    pub declarations: Vec<DeclarationEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
