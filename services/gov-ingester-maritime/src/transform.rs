//! Transform layer — maps the raw Maritime New Zealand legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, IncidentsEntity, TransformedCitizen, VesselsEntity},
    raw::RawMaritimeCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| IngestError::Transform(format!("invalid {field} date '{s}': {e}")))
}

pub fn transform_citizen(raw: &RawMaritimeCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with maritime_id {} has no DID",
            raw.maritime_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        maritime_id: raw.maritime_id.clone(),
    };

    let vessels = raw
        .vessels
        .iter()
        .map(|c| {
            Ok(VesselsEntity {
                vessel_name: c.vessel_name.clone(),
                flag: c.flag.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let incidents = raw
        .incidents
        .iter()
        .map(|c| {
            Ok(IncidentsEntity {
                reference: c.reference.clone(),
                incident_type: c.incident_type.clone(),
                incident_date: parse_date(&c.incident_date, "incident_date")?,
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        vessels,
        incidents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMaritimeCitizen {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/maritime_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawMaritimeBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.maritime_id, "MAR-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.vessels.first().unwrap().vessel_name, "MV Tane Moana");
        assert_eq!(t.vessels.first().unwrap().flag, "NZ");
        assert_eq!(t.vessels.first().unwrap().status, "registered");

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.incidents.first().unwrap().reference, "MAR-2026-02");
        assert_eq!(t.incidents.first().unwrap().incident_type, "Pollution");
        assert_eq!(
            t.incidents.first().unwrap().incident_date.to_string(),
            "2026-02-14"
        );
        assert_eq!(t.incidents.first().unwrap().status, "resolved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
