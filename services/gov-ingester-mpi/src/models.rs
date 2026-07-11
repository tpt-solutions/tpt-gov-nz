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
    pub mpi_id: String,
}

#[derive(Debug, Clone)]
pub struct RegistrationEntity {
    pub nzbn: String,
    pub business_name: String,
    pub r#type: String,
    pub status: String,
    pub registered_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct CertificationEntity {
    pub cert_number: String,
    pub category: String,
    pub issued_date: chrono::NaiveDate,
    pub expires_date: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub registrations: Vec<RegistrationEntity>,
    pub certifications: Vec<CertificationEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
