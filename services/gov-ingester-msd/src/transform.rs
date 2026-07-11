//! Transform layer — maps the raw MSD legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CaseEventEntity, CitizenEntity, StudyLinkEntity, TransformedCitizen},
    raw::RawMsdCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMsdCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with client number {} has no DID",
            raw.client_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        client_number: raw.client_number.clone(),
    };

    let studylink = match &raw.studylink {
        Some(s) => Some(StudyLinkEntity {
            has_student_loan: s.has_student_loan,
            loan_balance: s.loan_balance,
            repayment_plan: s.repayment_plan.clone(),
            has_allowance: s.has_allowance,
            allowance_type: s.allowance_type.clone(),
            next_payment_date: match &s.next_payment_date {
                Some(d) => Some(parse_date(d, "next payment date")?),
                None => None,
            },
            weekly_amount: s.weekly_amount,
        }),
        None => None,
    };

    let case_events = raw
        .case_history
        .iter()
        .map(|e| {
            Ok(CaseEventEntity {
                event_id: e.event_id.clone(),
                event_date: parse_date(&e.event_date, "event date")?,
                service_line: e.service_line.clone(),
                summary: e.summary.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        studylink,
        case_events,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMsdCitizen {
        serde_json::from_value(serde_json::json!({
            "clientNumber": "MSD-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "studylink": { "hasStudentLoan": true, "loanBalance": 28450.75, "repaymentPlan": "standard", "hasAllowance": true, "allowanceType": "living-allowance", "nextPaymentDate": "2026-07-15", "weeklyAmount": 221.48 },
            "caseHistory": [
                { "eventId": "MSD-EVT-001", "eventDate": "2026-05-02", "serviceLine": "Work and Income", "summary": "Applied for Jobseeker Support" },
                { "eventId": "MSD-EVT-002", "eventDate": "2026-06-10", "serviceLine": "StudyLink", "summary": "Student loan approved" }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_studylink() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_number, "MSD-100001");
        let sl = t.studylink.as_ref().unwrap();
        assert!(sl.has_student_loan);
        assert_eq!(sl.loan_balance, Some(28450.75));
        assert_eq!(sl.allowance_type.as_deref(), Some("living-allowance"));
        assert!(sl.next_payment_date.is_some());
    }

    #[test]
    fn maps_case_events() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.case_events.len(), 2);
        assert_eq!(t.case_events[0].event_id, "MSD-EVT-001");
        assert_eq!(t.case_events[1].service_line, "StudyLink");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_event_date() {
        let mut raw = sample();
        raw.case_history[0].event_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_next_payment_date() {
        let mut raw = sample();
        raw.studylink.as_mut().unwrap().next_payment_date = Some("soon".to_owned());
        assert!(transform_citizen(&raw).is_err());
    }
}
