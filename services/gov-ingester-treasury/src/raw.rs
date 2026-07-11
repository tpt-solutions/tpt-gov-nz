//! Raw The Treasury legacy data format.
//!
//! Mirrors the shape of a batch extract from the The Treasury legacy systems.
//! Distinct from the department `gov-dept-treasury` DB schema; the [`crate::transform`]
//! layer maps one to the other.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTreasuryBatch {
    pub batch_id: String,
    pub generated_at: String,
    pub source: String,
    pub citizens: Vec<RawTreasuryCitizen>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawTreasuryCitizen {
    pub treasury_id: String,
    pub did: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub budget: Vec<RawBudget>,
    #[serde(default)]
    pub economic_outlook: Option<RawEconomicOutlook>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawBudget {
    pub fiscal_year: i32,
    pub portfolio: String,
    pub appropriation: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawEconomicOutlook {
    pub forecast_year: i32,
    pub gdp_growth_pct: f64,
    pub inflation_pct: f64,
    pub net_debt_pct: f64,
}
