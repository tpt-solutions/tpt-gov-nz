//! Transform layer — maps the raw The Treasury legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{BudgetEntity, CitizenEntity, EconomicOutlookEntity, TransformedCitizen},
    raw::RawTreasuryCitizen,
};

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
                fiscal_year: c.fiscal_year,
                portfolio: c.portfolio.clone(),
                appropriation: c.appropriation.clone(),
                amount: c.amount,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let economic_outlook = raw
        .economic_outlook
        .as_ref()
        .map(|c| EconomicOutlookEntity {
            forecast_year: c.forecast_year,
            gdp_growth_pct: c.gdp_growth_pct,
            inflation_pct: c.inflation_pct,
            net_debt_pct: c.net_debt_pct,
        });

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
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/treasury_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawTreasuryBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.treasury_id, "TRE-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.budget.first().unwrap().fiscal_year, 2026);
        assert_eq!(t.budget.first().unwrap().portfolio, "Health");
        assert_eq!(t.budget.first().unwrap().appropriation, "Vote Health");
        assert_eq!(t.budget.first().unwrap().amount, 1200000000.0);

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.economic_outlook.as_ref().unwrap().forecast_year, 2026);
        assert_eq!(t.economic_outlook.as_ref().unwrap().gdp_growth_pct, 2.4);
        assert_eq!(t.economic_outlook.as_ref().unwrap().inflation_pct, 3.1);
        assert_eq!(t.economic_outlook.as_ref().unwrap().net_debt_pct, 42.0);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
