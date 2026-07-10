//! Raw MOH legacy data format (NHI / HPI systems).
//!
//! Mirrors the shape of a batch extract from the MOH legacy systems. It is
//! intentionally distinct from the department `gov-dept-moh` DB schema — the
//! [`crate::transform`] layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMohBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMohCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMohCitizen {
    pub nhi: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub gp_enrolment: Option<RawGpEnrolment>,
    #[serde(default)]
    pub prescriptions: Vec<RawPrescription>,
    #[serde(default)]
    pub appointments: Vec<RawAppointment>,
    #[serde(default)]
    pub vaccinations: Vec<RawVaccination>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawGpEnrolment {
    pub practice_name: String,
    pub address: String,
    pub phone: String,
    #[serde(default)]
    pub enrolled_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPrescription {
    pub medication: String,
    pub dose: String,
    #[serde(default)]
    pub repeats_total: i32,
    #[serde(default)]
    pub repeats_remaining: i32,
    pub issued_at: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawAppointment {
    pub provider: String,
    pub date: String,
    pub r#type: String,
    #[serde(default = "default_status")]
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawVaccination {
    pub vaccine: String,
    pub date: String,
    #[serde(default)]
    pub due_for_booster: bool,
}

fn default_status() -> String {
    "booked".to_owned()
}
