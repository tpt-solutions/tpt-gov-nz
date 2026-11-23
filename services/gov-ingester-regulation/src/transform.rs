//! Transform layer — maps the raw Ministry for Regulation legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, ProposalsEntity, RegulatoryReviewsEntity, TransformedCitizen},
    raw::RawRegulationCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| IngestError::Transform(format!("invalid {field} date '{s}': {e}")))
}

pub fn transform_citizen(raw: &RawRegulationCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with regulation_id {} has no DID",
            raw.regulation_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        regulation_id: raw.regulation_id.clone(),
    };

    let regulatory_reviews = raw
        .regulatory_reviews
        .iter()
        .map(|c| {
            Ok(RegulatoryReviewsEntity {
                topic: c.topic.clone(),
                agency: c.agency.clone(),
                status: c.status.clone(),
                review_year: c.review_year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let proposals = raw
        .proposals
        .iter()
        .map(|c| {
            Ok(ProposalsEntity {
                title: c.title.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        regulatory_reviews,
        proposals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawRegulationCitizen {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/regulation_batch.json")).unwrap();
                let batch: crate::raw::RawRegulationBatch = serde_json::from_slice(&bytes).unwrap();
                batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.regulation_id, "REG-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.regulatory_reviews.first().unwrap().topic,
            "Building consenting"
        );
        assert_eq!(t.regulatory_reviews.first().unwrap().agency, "MBIE");
        assert_eq!(t.regulatory_reviews.first().unwrap().status, "in-progress");
        assert_eq!(t.regulatory_reviews.first().unwrap().review_year, 2026);

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.proposals.first().unwrap().title,
            "Reduce duplicate reporting"
        );
        assert_eq!(t.proposals.first().unwrap().status, "consultation");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
