//! Transform layer — maps the raw MBIE legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        BusinessEntity, CitizenEntity, DirectorshipEntity, TransformedCitizen,
    },
    raw::RawMbieCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMbieCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with person id {} has no DID",
            raw.person_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        person_id: raw.person_id.clone(),
    };

    let businesses = raw
        .business_registrations
        .iter()
        .map(|b| {
            Ok(BusinessEntity {
                nzbn: b.nzbn.clone(),
                entity_name: b.entity_name.clone(),
                entity_type: b.entity_type.clone(),
                status: b.status.clone(),
                registered_date: parse_date(&b.registered_date, "registered date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let directorships = raw
        .directorships
        .iter()
        .map(|d| {
            Ok(DirectorshipEntity {
                nzbn: d.nzbn.clone(),
                entity_name: d.entity_name.clone(),
                role: d.role.clone(),
                appointed_date: parse_date(&d.appointed_date, "appointed date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        businesses,
        directorships,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMbieCitizen {
        serde_json::from_value(serde_json::json!({
            "personId": "MBIE-P-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "businessRegistrations": [{ "nzbn": "9429000000001", "entityName": "Tane Consulting Ltd", "entityType": "company", "status": "registered", "registeredDate": "2024-03-01" }],
            "directorships": [{ "nzbn": "9429000000001", "entityName": "Tane Consulting Ltd", "role": "Director", "appointedDate": "2024-03-01" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_business() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.person_id, "MBIE-P-100001");
        assert_eq!(t.businesses.len(), 1);
        assert_eq!(t.businesses[0].nzbn, "9429000000001");
        assert_eq!(t.businesses[0].entity_type, "company");
    }

    #[test]
    fn maps_directorships() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.directorships.len(), 1);
        assert_eq!(t.directorships[0].nzbn, "9429000000001");
        assert_eq!(t.directorships[0].role, "Director");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_registered_date() {
        let mut raw = sample();
        raw.business_registrations[0].registered_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
