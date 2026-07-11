//! Transform layer — maps the raw Ministry of Transport legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{StrategiesEntity,ProgrammesEntity,CitizenEntity, TransformedCitizen},
    raw::RawMotCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMotCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with mot_id {} has no DID",
            raw.mot_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        mot_id: raw.mot_id.clone(),
    };

    let strategies = raw
        .strategies
        .iter()
        .map(|c| {
            Ok(StrategiesEntity {
                title: c.title.clone(),
                year: c.year.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let programmes = raw
        .programmes
        .iter()
        .map(|c| {
            Ok(ProgrammesEntity {
                name: c.name.clone(),
                budget: c.budget.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        strategies,
        programmes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMotCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mot_id, "MOT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.strategies.as_ref().unwrap().title, "Te Tangi a Te Manu");
         assert_eq!(t.strategies.as_ref().unwrap().year, 2026);
         assert_eq!(t.strategies.as_ref().unwrap().status, "active");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.programmes.as_ref().unwrap().name, "Road maintenance boost");
         assert_eq!(t.programmes.as_ref().unwrap().budget, 800000000);
         assert_eq!(t.programmes.as_ref().unwrap().status, "funded");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
