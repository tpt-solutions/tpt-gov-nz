//! Transform layer — maps the raw Ministry of Transport legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, ProgrammesEntity, StrategiesEntity, TransformedCitizen},
    raw::RawMotCitizen,
};

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
                year: c.year,
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
                budget: c.budget,
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
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/mot_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawMotBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mot_id, "MOT-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.strategies.first().unwrap().title, "Te Tangi a Te Manu");
        assert_eq!(t.strategies.first().unwrap().year, 2026);
        assert_eq!(t.strategies.first().unwrap().status, "active");

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.programmes.first().unwrap().name, "Road maintenance boost");
        assert_eq!(t.programmes.first().unwrap().budget, 800000000.0);
        assert_eq!(t.programmes.first().unwrap().status, "funded");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
