//! Transform layer — maps the raw Fire and Emergency New Zealand legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{FireSafetyEntity,IncidentsEntity,CitizenEntity, TransformedCitizen},
    raw::RawFenzCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawFenzCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with fenz_id {} has no DID",
            raw.fenz_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        fenz_id: raw.fenz_id.clone(),
    };

    let fire_safety = match &raw.fire_safety {
        Some(c) => Some(FireSafetyEntity {
                property: c.property.clone(),
                grade: c.grade.clone(),
                last_inspection: parse_date(&c.last_inspection, "last_inspection")?,
        }),
        None => None,
    };

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
        fire_safety,
        incidents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawFenzCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.fenz_id, "FENZ-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.fire_safety.as_ref().unwrap().property, "12 Totara Street, Porirua");
         assert_eq!(t.fire_safety.as_ref().unwrap().grade, "Compliant");
         assert_eq!(t.fire_safety.as_ref().unwrap().last_inspection.to_string(), "2025-11-12");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.incidents.as_ref().unwrap().reference, "FENZ-2026-050");
         assert_eq!(t.incidents.as_ref().unwrap().incident_type, "Structure fire");
         assert_eq!(t.incidents.as_ref().unwrap().incident_date.to_string(), "2026-01-30");
         assert_eq!(t.incidents.as_ref().unwrap().status, "closed");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
