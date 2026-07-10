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
    pub passport_number: String,
}

#[derive(Debug, Clone)]
pub struct PassportEntity {
    pub passport_number: String,
    pub expiry_date: chrono::NaiveDate,
    pub renewable: bool,
}

#[derive(Debug, Clone)]
pub struct BirthCertEntity {
    pub certificate_number: String,
    pub date_of_birth: chrono::NaiveDate,
    pub place_of_birth: String,
    pub parents: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CitizenshipEntity {
    pub status: String,
    pub certificate_number: Option<String>,
    pub granted_at: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub passport: Option<PassportEntity>,
    pub birth_cert: Option<BirthCertEntity>,
    pub citizenship: Option<CitizenshipEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
