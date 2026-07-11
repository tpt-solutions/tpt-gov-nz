//! Transform layer — maps the raw The Treasury legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{BudgetEntity,EconomicOutlookEntity,CitizenEntity, TransformedCitizen},
    raw::RawTreasuryCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawTreasuryCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with treasury_id {} has no DID",
            raw.treasury_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        treasury_id: raw.treasury_id.clone(),
    };

    let budget = raw
        .budget
        .iter()
        .map(|c| {
            Ok(BudgetEntity {
                fiscal_year: c.fiscal_year.clone(),
                portfolio: c.portfolio.clone(),
                appropriation: c.appropriation.clone(),
                amount: c.amount.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let economic_outlook = match &raw.economic_outlook {
        Some(c) => Some(EconomicOutlookEntity {
                forecast_year: c.forecast_year.clone(),
                gdp_growth_pct: c.gdp_growth_pct.clone(),
                inflation_pct: c.inflation_pct.clone(),
                net_debt_pct: c.net_debt_pct.clone(),
        }),
        None => None,
    };

    Ok(TransformedCitizen {
        citizen,
        budget,
        economic_outlook,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawTreasuryCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.treasury_id, "TRE-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.budget.as_ref().unwrap().fiscal_year, 2026);
         assert_eq!(t.budget.as_ref().unwrap().portfolio, "Health");
         assert_eq!(t.budget.as_ref().unwrap().appropriation, "Vote Health");
         assert_eq!(t.budget.as_ref().unwrap().amount, 1200000000);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.economic_outlook.as_ref().unwrap().forecast_year, 2026);
         assert_eq!(t.economic_outlook.as_ref().unwrap().gdp_growth_pct, 2.4);
         assert_eq!(t.economic_outlook.as_ref().unwrap().inflation_pct, 3.1);
         assert_eq!(t.economic_outlook.as_ref().unwrap().net_debt_pct, 42);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
