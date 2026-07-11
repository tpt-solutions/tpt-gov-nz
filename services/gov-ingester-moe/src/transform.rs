//! Transform layer — maps the raw Ministry of Education legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{EnrolmentEntity,StudentSupportEntity,CitizenEntity, TransformedCitizen},
    raw::RawMoeCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMoeCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with moe_id {} has no DID",
            raw.moe_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        moe_id: raw.moe_id.clone(),
    };

    let enrolment = match &raw.enrolment {
        Some(c) => Some(EnrolmentEntity {
                school: c.school.clone(),
                year_level: c.year_level.clone(),
                status: c.status.clone(),
        }),
        None => None,
    };

    let student_support = raw
        .student_support
        .iter()
        .map(|c| {
            Ok(StudentSupportEntity {
                service: c.service.clone(),
                status: c.status.clone(),
                next_review: parse_date(&c.next_review, "next_review")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        enrolment,
        student_support,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMoeCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.moe_id, "MOE-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.enrolment.as_ref().unwrap().school, "Porirua College");
         assert_eq!(t.enrolment.as_ref().unwrap().year_level, 9);
         assert_eq!(t.enrolment.as_ref().unwrap().status, "enrolled");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.student_support.as_ref().unwrap().service, "Learning support");
         assert_eq!(t.student_support.as_ref().unwrap().status, "active");
         assert_eq!(t.student_support.as_ref().unwrap().next_review.to_string(), "2026-08-01");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
