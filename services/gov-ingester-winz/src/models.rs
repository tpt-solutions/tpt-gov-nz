use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
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
/// Produced by the transform layer from [`crate::raw::RawWinzCitizen`].
#[derive(Debug, Clone)]
pub struct CitizenEntity {
    pub did: String,
    pub client_id: String,
}

#[derive(Debug, Clone)]
pub struct BenefitEntity {
    pub benefit_type: String,
    pub weekly_amount: Decimal,
    pub start_date: Option<chrono::NaiveDate>,
    pub review_date: Option<chrono::NaiveDate>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct PaymentEntity {
    pub benefit_type: String,
    pub payment_date: chrono::NaiveDate,
    pub amount: Decimal,
    pub method: String,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub benefits: Vec<BenefitEntity>,
    pub payments: Vec<PaymentEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
