//! Transform layer — maps the raw Police legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, InfringementEntity, ReportEntity, TransformedCitizen},
    raw::RawPoliceCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawPoliceCitizen) -> Result<TransformedCitizen, IngestError> {
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

    let infringements = raw
        .infringements
        .iter()
        .map(|i| {
            Ok(InfringementEntity {
                ticket_number: i.ticket_number.clone(),
                offense_type: i.offense_type.clone(),
                status: i.status.clone(),
                amount: i.amount,
                issue_date: parse_date(&i.issue_date, "issue date")?,
                location: i.location.clone(),
                demerit_points: i.demerit_points,
                description: i.description.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let reports = raw
        .reports
        .iter()
        .map(|r| {
            Ok(ReportEntity {
                report_number: r.report_number.clone(),
                report_type: r.report_type.clone(),
                status: r.status.clone(),
                filed_date: parse_date(&r.filed_date, "filed date")?,
                description: r.description.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        infringements,
        reports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawPoliceCitizen {
        serde_json::from_value(serde_json::json!({
            "clientNumber": "POL-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "infringements": [{ "ticketNumber": "POL-T5001", "offenseType": "speeding", "status": "unpaid", "amount": 120, "issueDate": "2026-06-01", "location": "SH1", "demeritPoints": 20, "description": "Speeding" }],
            "reports": [{ "reportNumber": "POL-R2001", "reportType": "theft", "status": "under-investigation", "filedDate": "2026-05-20", "description": "Bicycle stolen" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_infringement() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_number, "POL-100001");
        assert_eq!(t.infringements.len(), 1);
        assert_eq!(t.infringements[0].ticket_number, "POL-T5001");
        assert_eq!(t.infringements[0].demerit_points, Some(20));
    }

    #[test]
    fn maps_report() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.reports.len(), 1);
        assert_eq!(t.reports[0].report_number, "POL-R2001");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_issue_date() {
        let mut raw = sample();
        raw.infringements[0].issue_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
