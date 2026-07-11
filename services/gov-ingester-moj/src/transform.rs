//! Transform layer — maps the raw MOJ legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        CitizenEntity, CourtRecordEntity, DisputeEntity, FineEntity, TransformedCitizen,
    },
    raw::RawMojCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMojCitizen) -> Result<TransformedCitizen, IngestError> {
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

    let fines = raw
        .fines
        .iter()
        .map(|f| {
            Ok(FineEntity {
                fine_number: f.fine_number.clone(),
                fine_type: f.fine_type.clone(),
                status: f.status.clone(),
                amount: f.amount,
                offense_date: parse_date(&f.offense_date, "offense date")?,
                due_date: parse_date(&f.due_date, "due date")?,
                description: f.description.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let disputes = raw
        .disputes
        .iter()
        .map(|d| DisputeEntity {
            dispute_number: d.dispute_number.clone(),
            claim_type: d.claim_type.clone(),
            status: d.status.clone(),
            amount_claimed: d.amount_claimed,
            hearing_date: d
                .hearing_date
                .as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
            description: d.description.clone(),
        })
        .collect::<Vec<_>>();

    let court_records = raw
        .court_records
        .iter()
        .map(|c| CourtRecordEntity {
            case_number: c.case_number.clone(),
            case_type: c.case_type.clone(),
            status: c.status.clone(),
            next_hearing_date: c
                .next_hearing_date
                .as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
            description: c.description.clone(),
        })
        .collect::<Vec<_>>();

    Ok(TransformedCitizen {
        citizen,
        fines,
        disputes,
        court_records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMojCitizen {
        serde_json::from_value(serde_json::json!({
            "clientNumber": "MOJ-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "fines": [{ "fineNumber": "MOJ-F5001", "fineType": "traffic", "status": "unpaid", "amount": 150, "offenseDate": "2026-05-01", "dueDate": "2026-06-15", "description": "Speeding" }],
            "disputes": [{ "disputeNumber": "MOJ-D2001", "claimType": "tenancy", "status": "filed", "amountClaimed": 1200, "hearingDate": "2026-08-20", "description": "Bond dispute" }],
            "courtRecords": [{ "caseNumber": "MOJ-C3001", "caseType": "traffic", "status": "open", "nextHearingDate": "2026-09-10", "description": "Careless driving" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_fine() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_number, "MOJ-100001");
        assert_eq!(t.fines.len(), 1);
        assert_eq!(t.fines[0].fine_number, "MOJ-F5001");
        assert_eq!(t.fines[0].amount, 150.0);
    }

    #[test]
    fn maps_dispute_and_court_record() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.disputes.len(), 1);
        assert_eq!(t.disputes[0].dispute_number, "MOJ-D2001");
        assert_eq!(t.court_records.len(), 1);
        assert_eq!(t.court_records[0].case_number, "MOJ-C3001");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_offense_date() {
        let mut raw = sample();
        raw.fines[0].offense_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
