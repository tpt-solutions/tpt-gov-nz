//! Raw Police legacy data format.
//!
//! Mirrors the shape of a batch extract from the Police legacy systems. Distinct
//! from the department `gov-dept-police` DB schema; the [`crate::transform`] layer
//! maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPoliceBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawPoliceCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawPoliceCitizen {
    pub client_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub infringements: Vec<RawInfringement>,
    #[serde(default)]
    pub reports: Vec<RawReport>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawInfringement {
    pub ticket_number: String,
    pub offense_type: String,
    pub status: String,
    pub amount: f64,
    pub issue_date: String,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub demerit_points: Option<i32>,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawReport {
    pub report_number: String,
    pub report_type: String,
    pub status: String,
    pub filed_date: String,
    pub description: String,
}
