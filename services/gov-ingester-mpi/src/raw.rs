//! Raw MPI legacy data format.
//!
//! Mirrors the shape of a batch extract from the MPI legacy systems.
//! Distinct from the department `gov-dept-mpi` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMpiBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawMpiCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMpiCitizen {
    pub mpi_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub registrations: Vec<RawRegistration>,
    #[serde(default)]
    pub certifications: Vec<RawCertification>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRegistration {
    pub nzbn: String,
    pub business_name: String,
    pub r#type: String,
    pub status: String,
    pub registered_date: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCertification {
    pub cert_number: String,
    pub category: String,
    pub issued_date: String,
    pub expires_date: String,
}
