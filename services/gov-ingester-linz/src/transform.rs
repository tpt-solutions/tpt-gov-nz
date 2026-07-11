//! Transform layer — maps the raw LINZ legacy format to the department DB schema.
//!
//! Titles and ownership records carry no dates, so the mapping is a straight
//! field copy; `registered_owners` is serialised to a JSON value for the jsonb
//! column.

use crate::{
    error::IngestError,
    models::{CitizenEntity, OwnershipEntity, TitleEntity, TransformedCitizen},
    raw::RawLinzCitizen,
};

pub fn transform_citizen(raw: &RawLinzCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with customer id {} has no DID",
            raw.customer_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        customer_id: raw.customer_id.clone(),
    };

    let titles = raw
        .titles
        .iter()
        .map(|t| TitleEntity {
            title_number: t.title_number.clone(),
            property_address: t.property_address.clone(),
            land_area_sqm: t.land_area_sqm,
            estate_type: t.estate_type.clone(),
        })
        .collect::<Vec<_>>();

    let ownership = raw
        .ownership
        .iter()
        .map(|o| OwnershipEntity {
            title_number: o.title_number.clone(),
            ownership_share: o.ownership_share.clone(),
            registered_owners: serde_json::json!(o.registered_owners),
        })
        .collect::<Vec<_>>();

    Ok(TransformedCitizen {
        citizen,
        titles,
        ownership,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawLinzCitizen {
        serde_json::from_value(serde_json::json!({
            "customerId": "LINZ-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "titles": [{ "titleNumber": "LNZ-T-1", "propertyAddress": "45 Kahu Road, Wellington", "landAreaSqm": 612.5, "estateType": "Freehold" }],
            "ownership": [{ "titleNumber": "LNZ-T-1", "ownershipShare": "1/1", "registeredOwners": ["Alex Tane"] }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_title() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.customer_id, "LINZ-100001");
        assert_eq!(t.titles.len(), 1);
        assert_eq!(t.titles[0].title_number, "LNZ-T-1");
        assert_eq!(t.titles[0].land_area_sqm, 612.5);
    }

    #[test]
    fn maps_ownership() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.ownership.len(), 1);
        assert_eq!(t.ownership[0].title_number, "LNZ-T-1");
        assert_eq!(t.ownership[0].ownership_share, "1/1");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn empty_citizen_is_valid_without_records() {
        let raw: RawLinzCitizen = serde_json::from_value(serde_json::json!({
            "customerId": "LINZ-9", "did": "did:gov:nz:t", "titles": [], "ownership": []
        }))
        .unwrap();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.titles.len(), 0);
        assert_eq!(t.ownership.len(), 0);
    }
}
