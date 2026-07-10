use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Result of a single ingestion pass. Carried through to the audit log.
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

/// Typed, schema-aligned view of one citizen, ready to upsert.
#[derive(Debug, Clone)]
pub struct CitizenEntity {
    pub did: String,
    pub nhi: String,
}

#[derive(Debug, Clone)]
pub struct GpEntity {
    pub practice_name: String,
    pub address: String,
    pub phone: String,
    pub enrolled_at: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct PrescriptionEntity {
    pub medication: String,
    pub dose: String,
    pub repeats_total: i32,
    pub repeats_remaining: i32,
    pub issued_at: chrono::NaiveDate,
}

#[derive(Debug, Clone)]
pub struct AppointmentEntity {
    pub provider: String,
    pub appt_date: chrono::DateTime<chrono::Utc>,
    pub r#type: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct VaccinationEntity {
    pub vaccine: String,
    pub vaccine_date: chrono::NaiveDate,
    pub due_for_booster: bool,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub gp: Option<GpEntity>,
    pub prescriptions: Vec<PrescriptionEntity>,
    pub appointments: Vec<AppointmentEntity>,
    pub vaccinations: Vec<VaccinationEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
