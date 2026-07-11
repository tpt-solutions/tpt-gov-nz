//! Transform layer — maps the raw Education Review Office legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ReviewsEntity,ReportsEntity,CitizenEntity, TransformedCitizen},
    raw::RawEroCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawEroCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with ero_id {} has no DID",
            raw.ero_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        ero_id: raw.ero_id.clone(),
    };

    let reviews = raw
        .reviews
        .iter()
        .map(|c| {
            Ok(ReviewsEntity {
                school: c.school.clone(),
                rating: c.rating.clone(),
                review_date: parse_date(&c.review_date, "review_date")?,
                next_review: parse_date(&c.next_review, "next_review")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let reports = raw
        .reports
        .iter()
        .map(|c| {
            Ok(ReportsEntity {
                title: c.title.clone(),
                published: parse_date(&c.published, "published")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        reviews,
        reports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawEroCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.ero_id, "ERO-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.reviews.as_ref().unwrap().school, "Porirua College");
         assert_eq!(t.reviews.as_ref().unwrap().rating, "Developing");
         assert_eq!(t.reviews.as_ref().unwrap().review_date.to_string(), "2025-09-01");
         assert_eq!(t.reviews.as_ref().unwrap().next_review.to_string(), "2027-09-01");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.reports.as_ref().unwrap().title, "Porirua College annual report");
         assert_eq!(t.reports.as_ref().unwrap().published.to_string(), "2025-10-15");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
