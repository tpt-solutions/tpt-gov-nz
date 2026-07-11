//! Transform layer — maps the raw NZQA legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, QualificationEntity, TranscriptEntity, TransformedCitizen},
    raw::RawNzqaCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawNzqaCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with NSN {} has no DID",
            raw.nsn
        )));
    }

    if raw.nsn.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with DID {} has no NSN",
            raw.did
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        nsn: raw.nsn.clone(),
    };

    let qualifications = raw
        .qualifications
        .iter()
        .map(|q| {
            Ok(QualificationEntity {
                qualification_id: q.qualification_id.clone(),
                title: q.title.clone(),
                level: q.level,
                awarded_date: parse_date(&q.awarded_date, "awarded date")?,
                provider: q.provider.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let transcript = raw.transcript.as_ref().map(|t| TranscriptEntity {
        record_summary: t.record_summary.clone(),
        total_credits: t.total_credits,
        credit_summary: t.credit_summary.clone(),
    });

    Ok(TransformedCitizen {
        citizen,
        qualifications,
        transcript,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawNzqaCitizen {
        serde_json::from_value(serde_json::json!({
            "nsn": "NSN-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "qualifications": [{ "qualificationId": "NZQA-Q1", "title": "Bachelor of Science", "level": 7, "awardedDate": "2024-12-10", "provider": "University of Auckland" }],
            "transcript": { "recordSummary": "Full Record of Achievement on file.", "totalCredits": 360, "creditSummary": "Level 7: 360 credits" }
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_qualification() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.nsn, "NSN-100001");
        assert_eq!(t.qualifications.len(), 1);
        assert_eq!(t.qualifications[0].qualification_id, "NZQA-Q1");
        assert_eq!(t.qualifications[0].level, 7);
    }

    #[test]
    fn maps_transcript() {
        let t = transform_citizen(&sample()).unwrap();
        let tr = t.transcript.unwrap();
        assert_eq!(tr.total_credits, Some(360));
        assert_eq!(tr.record_summary, Some("Full Record of Achievement on file.".to_owned()));
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_awarded_date() {
        let mut raw = sample();
        raw.qualifications[0].awarded_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
