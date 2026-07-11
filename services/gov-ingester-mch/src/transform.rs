//! Transform layer — maps the raw Ministry for Culture and Heritage legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{HeritageSitesEntity,GrantsEntity,CitizenEntity, TransformedCitizen},
    raw::RawMchCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMchCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with mch_id {} has no DID",
            raw.mch_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        mch_id: raw.mch_id.clone(),
    };

    let heritage_sites = raw
        .heritage_sites
        .iter()
        .map(|c| {
            Ok(HeritageSitesEntity {
                name: c.name.clone(),
                status: c.status.clone(),
                region: c.region.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let grants = raw
        .grants
        .iter()
        .map(|c| {
            Ok(GrantsEntity {
                grant_name: c.grant_name.clone(),
                amount: c.amount.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        heritage_sites,
        grants,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMchCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mch_id, "MCH-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.heritage_sites.as_ref().unwrap().name, "Old St Paul's");
         assert_eq!(t.heritage_sites.as_ref().unwrap().status, "Category 1 historic place");
         assert_eq!(t.heritage_sites.as_ref().unwrap().region, "Wellington");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.grants.as_ref().unwrap().grant_name, "Cultural Innovation Fund");
         assert_eq!(t.grants.as_ref().unwrap().amount, 15000);
         assert_eq!(t.grants.as_ref().unwrap().status, "approved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
