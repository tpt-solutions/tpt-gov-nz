//! Transform layer — maps the raw Ministry of Foreign Affairs and Trade legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{OverseasMissionsEntity,TravelAdvisoriesEntity,CitizenEntity, TransformedCitizen},
    raw::RawMfatCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMfatCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with mfat_id {} has no DID",
            raw.mfat_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        mfat_id: raw.mfat_id.clone(),
    };

    let overseas_missions = raw
        .overseas_missions
        .iter()
        .map(|c| {
            Ok(OverseasMissionsEntity {
                country: c.country.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let travel_advisories = raw
        .travel_advisories
        .iter()
        .map(|c| {
            Ok(TravelAdvisoriesEntity {
                country: c.country.clone(),
                level: c.level.clone(),
                updated: parse_date(&c.updated, "updated")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        overseas_missions,
        travel_advisories,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMfatCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mfat_id, "MFAT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.overseas_missions.as_ref().unwrap().country, "Australia");
         assert_eq!(t.overseas_missions.as_ref().unwrap().status, "active");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.travel_advisories.as_ref().unwrap().country, "Indonesia");
         assert_eq!(t.travel_advisories.as_ref().unwrap().level, "Exercise increased caution");
         assert_eq!(t.travel_advisories.as_ref().unwrap().updated.to_string(), "2026-03-10");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
