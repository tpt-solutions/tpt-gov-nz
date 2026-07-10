//! Raw DIA legacy data format.
//!
//! Mirrors the shape of a batch extract from the DIA legacy systems. Distinct from
//! the department `gov-dept-dia` DB schema; the [`crate::transform`] layer maps one
//! to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDiaBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawDiaCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawDiaCitizen {
    pub passport_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub passport: Option<RawPassport>,
    #[serde(default)]
    pub birth_cert: Option<RawBirthCert>,
    #[serde(default)]
    pub citizenship: Option<RawCitizenship>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPassport {
    pub expiry_date: String,
    #[serde(default = "default_renewable")]
    pub renewable: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawBirthCert {
    pub certificate_number: String,
    pub date_of_birth: String,
    pub place_of_birth: String,
    #[serde(default)]
    pub parents: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawCitizenship {
    pub status: String,
    #[serde(default)]
    pub certificate_number: Option<String>,
    #[serde(default)]
    pub granted_at: Option<String>,
}

fn default_renewable() -> bool {
    true
}
