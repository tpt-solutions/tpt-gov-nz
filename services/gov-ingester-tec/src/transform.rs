//! Transform layer — maps the raw Tertiary Education Commission legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, CoursesEntity, FundingEntity, TransformedCitizen},
    raw::RawTecCitizen,
};

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
                amount: c.amount,
                year: c.year,
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
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/tec_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawTecBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.tec_id, "TEC-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.funding.first().unwrap().provider, "Whitireia");
        assert_eq!(t.funding.first().unwrap().amount, 2200000.0);
        assert_eq!(t.funding.first().unwrap().year, 2026);

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.courses.first().unwrap().course_name,
            "New Zealand Certificate in IT"
        );
        assert_eq!(t.courses.first().unwrap().provider, "Whitireia");
        assert_eq!(t.courses.first().unwrap().status, "approved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
