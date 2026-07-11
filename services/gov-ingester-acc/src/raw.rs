//! Raw ACC legacy data format.
//!
//! Mirrors the shape of a batch extract from the ACC legacy systems. Distinct from
//! the department `gov-dept-acc` DB schema; the [`crate::transform`] layer maps
//! one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawAccBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawAccCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawAccCitizen {
    pub client_number: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub claims: Vec<RawClaim>,
    #[serde(default)]
    pub entitlements: Option<RawEntitlement>,
    #[serde(default)]
    pub rehabilitation: Vec<RawRehabilitation>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawClaim {
    pub claim_number: String,
    pub claim_type: String,
    pub status: String,
    pub injury_date: String,
    pub description: String,
    #[serde(default)]
    pub weekly_compensation: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEntitlement {
    pub has_entitlement: bool,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub weekly_amount: Option<f64>,
    #[serde(default)]
    pub remaining_weeks: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawRehabilitation {
    pub plan_id: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub next_review: Option<String>,
}
