//! Transform layer — maps the raw Corrections legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CaseEntity, CitizenEntity, ProbationEntity, TransformedCitizen},
    raw::RawCorrectionsCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawCorrectionsCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with corrections id {} has no DID",
            raw.corrections_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        corrections_id: raw.corrections_id.clone(),
    };

    let probation = raw.probation.as_ref().map(|p| {
        Ok::<_, IngestError>(ProbationEntity {
            status: p.status.clone(),
            officer_name: p.officer_name.clone(),
            next_report_date: parse_date(&p.next_report_date, "next report date")?,
            location: p.location.clone(),
        })
    }).transpose()?;

    let cases = raw
        .case
        .iter()
        .map(|c| {
            Ok(CaseEntity {
                case_number: c.case_number.clone(),
                sentence_type: c.sentence_type.clone(),
                start_date: parse_date(&c.start_date, "start date")?,
                end_date: match c.end_date.as_deref() {
                    Some("") | None => None,
                    Some(s) => Some(parse_date(s, "end date")?),
                },
                summary: c.summary.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        probation,
        cases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawCorrectionsCitizen {
        serde_json::from_value(serde_json::json!({
            "correctionsId": "COR-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "probation": { "status": "active", "officerName": "Officer R. Reedy", "nextReportDate": "2026-08-15", "location": "Auckland Probation Hub" },
            "case": [{ "caseNumber": "COR-C0001", "sentenceType": "supervision", "startDate": "2025-03-01", "endDate": "2027-03-01", "summary": "Supervision sentence." }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_probation() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.corrections_id, "COR-100001");
        assert!(t.probation.is_some());
        assert_eq!(t.probation.as_ref().unwrap().officer_name, "Officer R. Reedy");
    }

    #[test]
    fn maps_case() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.cases.len(), 1);
        assert_eq!(t.cases[0].case_number, "COR-C0001");
        assert_eq!(t.cases[0].end_date, Some(chrono::NaiveDate::from_ymd_opt(2027, 3, 1).unwrap()));
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_start_date() {
        let mut raw = sample();
        raw.case[0].start_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
