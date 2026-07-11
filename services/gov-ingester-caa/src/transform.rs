//! Transform layer — maps the raw Civil Aviation Authority legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{LicencesEntity,AircraftEntity,CitizenEntity, TransformedCitizen},
    raw::RawCaaCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawCaaCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with caa_id {} has no DID",
            raw.caa_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        caa_id: raw.caa_id.clone(),
    };

    let licences = raw
        .licences
        .iter()
        .map(|c| {
            Ok(LicencesEntity {
                licence_no: c.licence_no.clone(),
                category: c.category.clone(),
                status: c.status.clone(),
                expires: parse_date(&c.expires, "expires")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let aircraft = raw
        .aircraft
        .iter()
        .map(|c| {
            Ok(AircraftEntity {
                registration: c.registration.clone(),
                aircraft_type: c.aircraft_type.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        licences,
        aircraft,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawCaaCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.caa_id, "CAA-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.licences.as_ref().unwrap().licence_no, "CAA-P-55821");
         assert_eq!(t.licences.as_ref().unwrap().category, "Private Pilot");
         assert_eq!(t.licences.as_ref().unwrap().status, "current");
         assert_eq!(t.licences.as_ref().unwrap().expires.to_string(), "2027-06-30");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.aircraft.as_ref().unwrap().registration, "ZK-TAN");
         assert_eq!(t.aircraft.as_ref().unwrap().aircraft_type, "Cessna 172");
         assert_eq!(t.aircraft.as_ref().unwrap().status, "registered");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
