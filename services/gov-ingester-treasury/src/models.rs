use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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
    pub treasury_id: String,
}

#[derive(Debug, Clone)]
pub struct BudgetEntity {
    pub fiscal_year: i32,
    pub portfolio: String,
    pub appropriation: String,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct EconomicOutlookEntity {
    pub forecast_year: i32,
    pub gdp_growth_pct: f64,
    pub inflation_pct: f64,
    pub net_debt_pct: f64,
}

#[derive(Debug, Clone)]
pub struct TransformedCitizen {
    pub citizen: CitizenEntity,
    pub budget: Vec<BudgetEntity>,
    pub economic_outlook: Option<EconomicOutlookEntity>,
}

pub fn new_run_id() -> Uuid {
    Uuid::new_v4()
}
