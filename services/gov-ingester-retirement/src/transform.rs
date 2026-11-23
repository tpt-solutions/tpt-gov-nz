//! Transform layer — maps the raw Retirement Commission (Te Ara Ahunga Ora) legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, GuidanceEntity, RetirementPlanEntity, TransformedCitizen},
    raw::RawRetirementCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| IngestError::Transform(format!("invalid {field} date '{s}': {e}")))
}

pub fn transform_citizen(raw: &RawRetirementCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with retirement_id {} has no DID",
            raw.retirement_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        retirement_id: raw.retirement_id.clone(),
    };

    let retirement_plan = match &raw.retirement_plan {
        Some(c) => Some(RetirementPlanEntity {
            has_plan: c.has_plan,
            retirement_age: c.retirement_age,
            last_review: parse_date(&c.last_review, "last_review")?,
        }),
        None => None,
    };

    let guidance = raw
        .guidance
        .iter()
        .map(|c| {
            Ok(GuidanceEntity {
                topic: c.topic.clone(),
                summary: c.summary.clone(),
                published: parse_date(&c.published, "published")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        retirement_plan,
        guidance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawRetirementCitizen {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/retirement_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawRetirementBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.retirement_id, "RET-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert!(t.retirement_plan.as_ref().unwrap().has_plan);
        assert_eq!(t.retirement_plan.as_ref().unwrap().retirement_age, 65);
        assert_eq!(
            t.retirement_plan.as_ref().unwrap().last_review.to_string(),
            "2025-12-01"
        );

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.guidance.first().unwrap().topic,
            "KiwiSaver contribution rate"
        );
        assert_eq!(
            t.guidance.first().unwrap().summary,
            "Consider increasing to 6%."
        );
        assert_eq!(
            t.guidance.first().unwrap().published.to_string(),
            "2026-02-20"
        );
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
