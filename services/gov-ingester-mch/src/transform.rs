//! Transform layer — maps the raw Ministry for Culture and Heritage legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, GrantsEntity, HeritageSitesEntity, TransformedCitizen},
    raw::RawMchCitizen,
};

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
                amount: c.amount,
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
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/mch_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawMchBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mch_id, "MCH-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.heritage_sites.first().unwrap().name, "Old St Paul's");
        assert_eq!(
            t.heritage_sites.first().unwrap().status,
            "Category 1 historic place"
        );
        assert_eq!(t.heritage_sites.first().unwrap().region, "Wellington");

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.grants.first().unwrap().grant_name,
            "Cultural Innovation Fund"
        );
        assert_eq!(t.grants.first().unwrap().amount, 15000.0);
        assert_eq!(t.grants.first().unwrap().status, "approved");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
