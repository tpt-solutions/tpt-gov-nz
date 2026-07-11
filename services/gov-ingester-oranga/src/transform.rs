//! Transform layer — maps the raw Oranga Tamariki legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CarePlacementsEntity,SupportServicesEntity,CitizenEntity, TransformedCitizen},
    raw::RawOrangaCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawOrangaCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with oranga_id {} has no DID",
            raw.oranga_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        oranga_id: raw.oranga_id.clone(),
    };

    let care_placements = raw
        .care_placements
        .iter()
        .map(|c| {
            Ok(CarePlacementsEntity {
                placement_type: c.placement_type.clone(),
                start_date: parse_date(&c.start_date, "start_date")?,
                region: c.region.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let support_services = raw
        .support_services
        .iter()
        .map(|c| {
            Ok(SupportServicesEntity {
                service: c.service.clone(),
                status: c.status.clone(),
                next_review: parse_date(&c.next_review, "next_review")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        care_placements,
        support_services,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawOrangaCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.oranga_id, "OT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.care_placements.as_ref().unwrap().placement_type, "Whānau placement");
         assert_eq!(t.care_placements.as_ref().unwrap().start_date.to_string(), "2025-08-01");
         assert_eq!(t.care_placements.as_ref().unwrap().region, "Waikato");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.support_services.as_ref().unwrap().service, "Intensive support");
         assert_eq!(t.support_services.as_ref().unwrap().status, "active");
         assert_eq!(t.support_services.as_ref().unwrap().next_review.to_string(), "2026-09-01");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
