//! Transform layer — maps the raw TPK legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, FundingEntity, ProgrammeEntity, TransformedCitizen},
    raw::RawTpkCitizen,
};

pub fn transform_citizen(raw: &RawTpkCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with tpk id {} has no DID",
            raw.tpk_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        tpk_id: raw.tpk_id.clone(),
    };

    let programmes = raw
        .programmes
        .iter()
        .map(|p| Ok(ProgrammeEntity {
            programme_name: p.programme_name.clone(),
            status: p.status.clone(),
            region: p.region.clone(),
        }))
        .collect::<Result<Vec<_>, IngestError>>()?;

    let funding = raw
        .funding
        .iter()
        .map(|f| {
            if f.amount < 0 {
                return Err(IngestError::Transform(format!(
                    "funding {} has a negative amount",
                    f.grant_id
                )));
            }
            Ok(FundingEntity {
                grant_id: f.grant_id.clone(),
                amount: f.amount,
                purpose: f.purpose.clone(),
                status: f.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        programmes,
        funding,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawTpkCitizen {
        serde_json::from_value(serde_json::json!({
            "tpkId": "TPK-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "programmes": [{ "programmeName": "Te Hono", "status": "enrolled", "region": "Te Tai Tokerau" }],
            "funding": [{ "grantId": "TPK-G1001", "amount": 5000, "purpose": "Marae renovations", "status": "approved" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_programme() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.tpk_id, "TPK-100001");
        assert_eq!(t.programmes.len(), 1);
        assert_eq!(t.programmes[0].programme_name, "Te Hono");
        assert_eq!(t.programmes[0].region, "Te Tai Tokerau");
    }

    #[test]
    fn maps_funding() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.funding.len(), 1);
        assert_eq!(t.funding[0].grant_id, "TPK-G1001");
        assert_eq!(t.funding[0].amount, 5000);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_negative_amount() {
        let mut raw = sample();
        raw.funding[0].amount = -1;
        assert!(transform_citizen(&raw).is_err());
    }
}
