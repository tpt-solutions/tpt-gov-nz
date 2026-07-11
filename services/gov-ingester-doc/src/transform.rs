//! Transform layer — maps the raw DOC legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, ConcessionEntity, PermitEntity, TransformedCitizen},
    raw::RawDocCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawDocCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with doc id {} has no DID",
            raw.doc_id
        )));
    }

    if raw.doc_id.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with DID {} has no doc id",
            raw.did
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        doc_id: raw.doc_id.clone(),
    };

    let permits = raw
        .permits
        .iter()
        .map(|p| {
            Ok(PermitEntity {
                permit_number: p.permit_number.clone(),
                activity: p.activity.clone(),
                location: p.location.clone(),
                status: p.status.clone(),
                expires_date: parse_date(&p.expires_date, "expires date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let concessions = raw
        .concessions
        .iter()
        .map(|c| {
            Ok(ConcessionEntity {
                concession_id: c.concession_id.clone(),
                r#type: c.r#type.clone(),
                holder: c.holder.clone(),
                start_date: parse_date(&c.start_date, "start date")?,
                end_date: parse_date(&c.end_date, "end date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        permits,
        concessions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawDocCitizen {
        serde_json::from_value(serde_json::json!({
            "docId": "DOC-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "permits": [{ "permitNumber": "DOC-P1001", "activity": "Recreational hunting", "location": "Whakarewarewa Forest", "status": "active", "expiresDate": "2027-05-01" }],
            "concessions": [{ "concessionId": "DOC-C2001", "type": "guided-tour", "holder": "Alex Tane", "startDate": "2026-01-01", "endDate": "2026-12-31" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_permit() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.doc_id, "DOC-100001");
        assert_eq!(t.permits.len(), 1);
        assert_eq!(t.permits[0].permit_number, "DOC-P1001");
        assert_eq!(t.permits[0].expires_date.to_string(), "2027-05-01");
    }

    #[test]
    fn maps_concession() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.concessions.len(), 1);
        assert_eq!(t.concessions[0].concession_id, "DOC-C2001");
        assert_eq!(t.concessions[0].end_date.to_string(), "2026-12-31");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_expires_date() {
        let mut raw = sample();
        raw.permits[0].expires_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
