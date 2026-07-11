//! Transform layer — maps the raw Ministry for Ethnic Communities legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ProgrammesEntity,CommunityGrantsEntity,CitizenEntity, TransformedCitizen},
    raw::RawEthnicCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawEthnicCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with ethnic_id {} has no DID",
            raw.ethnic_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        ethnic_id: raw.ethnic_id.clone(),
    };

    let programmes = raw
        .programmes
        .iter()
        .map(|c| {
            Ok(ProgrammesEntity {
                programme_name: c.programme_name.clone(),
                status: c.status.clone(),
                year: c.year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let community_grants = raw
        .community_grants
        .iter()
        .map(|c| {
            Ok(CommunityGrantsEntity {
                grant_name: c.grant_name.clone(),
                amount: c.amount.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        programmes,
        community_grants,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawEthnicCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.ethnic_id, "ETH-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.programmes.as_ref().unwrap().programme_name, "Ethnic Communities Graduate Programme");
         assert_eq!(t.programmes.as_ref().unwrap().status, "enrolled");
         assert_eq!(t.programmes.as_ref().unwrap().year, 2026);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.community_grants.as_ref().unwrap().grant_name, "Community-led response fund");
         assert_eq!(t.community_grants.as_ref().unwrap().amount, 5000);
         assert_eq!(t.community_grants.as_ref().unwrap().status, "approved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
