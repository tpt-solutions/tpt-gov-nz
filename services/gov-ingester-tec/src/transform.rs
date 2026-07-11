//! Transform layer — maps the raw Tertiary Education Commission legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{FundingEntity,CoursesEntity,CitizenEntity, TransformedCitizen},
    raw::RawTecCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawTecCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with tec_id {} has no DID",
            raw.tec_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        tec_id: raw.tec_id.clone(),
    };

    let funding = raw
        .funding
        .iter()
        .map(|c| {
            Ok(FundingEntity {
                provider: c.provider.clone(),
                amount: c.amount.clone(),
                year: c.year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let courses = raw
        .courses
        .iter()
        .map(|c| {
            Ok(CoursesEntity {
                course_name: c.course_name.clone(),
                provider: c.provider.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        funding,
        courses,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawTecCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.tec_id, "TEC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.funding.as_ref().unwrap().provider, "Whitireia");
         assert_eq!(t.funding.as_ref().unwrap().amount, 2200000);
         assert_eq!(t.funding.as_ref().unwrap().year, 2026);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.courses.as_ref().unwrap().course_name, "New Zealand Certificate in IT");
         assert_eq!(t.courses.as_ref().unwrap().provider, "Whitireia");
         assert_eq!(t.courses.as_ref().unwrap().status, "approved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
