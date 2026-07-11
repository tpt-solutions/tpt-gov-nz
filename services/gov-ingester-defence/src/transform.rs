//! Transform layer — maps the raw Ministry of Defence legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ProcurementsEntity,BasesEntity,CitizenEntity, TransformedCitizen},
    raw::RawDefenceCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawDefenceCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with defence_id {} has no DID",
            raw.defence_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        defence_id: raw.defence_id.clone(),
    };

    let procurements = raw
        .procurements
        .iter()
        .map(|c| {
            Ok(ProcurementsEntity {
                programme: c.programme.clone(),
                value: c.value.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let bases = raw
        .bases
        .iter()
        .map(|c| {
            Ok(BasesEntity {
                name: c.name.clone(),
                location: c.location.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        procurements,
        bases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawDefenceCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.defence_id, "DEF-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.procurements.as_ref().unwrap().programme, "Frigate sustainment");
         assert_eq!(t.procurements.as_ref().unwrap().value, 450000000);
         assert_eq!(t.procurements.as_ref().unwrap().status, "ongoing");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.bases.as_ref().unwrap().name, "Trentham Military Camp");
         assert_eq!(t.bases.as_ref().unwrap().location, "Upper Hutt");
         assert_eq!(t.bases.as_ref().unwrap().status, "operational");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
