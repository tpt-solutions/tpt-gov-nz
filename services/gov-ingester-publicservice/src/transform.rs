//! Transform layer — maps the raw Te Kawa Mataaho Public Service Commission legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{WorkforceEntity,AgencyRatingsEntity,CitizenEntity, TransformedCitizen},
    raw::RawPublicserviceCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawPublicserviceCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with psc_id {} has no DID",
            raw.psc_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        psc_id: raw.psc_id.clone(),
    };

    let workforce = raw
        .workforce
        .iter()
        .map(|c| {
            Ok(WorkforceEntity {
                report_year: c.report_year.clone(),
                agency: c.agency.clone(),
                headcount: c.headcount.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let agency_ratings = raw
        .agency_ratings
        .iter()
        .map(|c| {
            Ok(AgencyRatingsEntity {
                agency: c.agency.clone(),
                rating: c.rating.clone(),
                rating_year: c.rating_year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        workforce,
        agency_ratings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawPublicserviceCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.psc_id, "PSC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.workforce.as_ref().unwrap().report_year, 2025);
         assert_eq!(t.workforce.as_ref().unwrap().agency, "Department of Internal Affairs");
         assert_eq!(t.workforce.as_ref().unwrap().headcount, 4200);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.agency_ratings.as_ref().unwrap().agency, "Department of Internal Affairs");
         assert_eq!(t.agency_ratings.as_ref().unwrap().rating, "Good");
         assert_eq!(t.agency_ratings.as_ref().unwrap().rating_year, 2025);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
