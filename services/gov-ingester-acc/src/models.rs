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
    pub client_number: String,
}

#[derive(Debug, Clone)]
pub struct ClaimEntity {
    pub claim_number: String,
    pub claim_type: String,
    pub status: String,
    pub injury_date: chrono::NaiveDate,
    pub description: String,
    pub weekly_compensation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct EntitlementEntity {
    pub has_entitlement: bool,
    pub r#type: Option<String>,
    pub weekly_amount: Option<f64>,
    pub remaining_weeks: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct RehabilitationEntity {
    pub plan_id: String,
    pub description: String,
    pub status: String,
    pub provider: Option<String>,
    pub next_review: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub claims: Vec<ClaimEntity>,
    pub entitlements: Option<EntitlementEntity>,
    pub rehabilitation: Vec<RehabilitationEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
