//! Transform layer — maps the raw Earthquake Commission (Toka Tū Ake) legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ClaimsEntity,CoverEntity,CitizenEntity, TransformedCitizen},
    raw::RawEqcCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawEqcCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with eqc_id {} has no DID",
            raw.eqc_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        eqc_id: raw.eqc_id.clone(),
    };

    let claims = raw
        .claims
        .iter()
        .map(|c| {
            Ok(ClaimsEntity {
                reference: c.reference.clone(),
                property: c.property.clone(),
                status: c.status.clone(),
                lodged_date: parse_date(&c.lodged_date, "lodged_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let cover = match &raw.cover {
        Some(c) => Some(CoverEntity {
                property: c.property.clone(),
                sum_insured: c.sum_insured.clone(),
                valid_to: parse_date(&c.valid_to, "valid_to")?,
        }),
        None => None,
    };

    Ok(TransformedCitizen {
        citizen,
        claims,
        cover,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawEqcCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.eqc_id, "EQC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.claims.as_ref().unwrap().reference, "EQC-2026-007");
         assert_eq!(t.claims.as_ref().unwrap().property, "12 Totara Street, Porirua");
         assert_eq!(t.claims.as_ref().unwrap().status, "assessed");
         assert_eq!(t.claims.as_ref().unwrap().lodged_date.to_string(), "2026-03-02");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.cover.as_ref().unwrap().property, "12 Totara Street, Porirua");
         assert_eq!(t.cover.as_ref().unwrap().sum_insured, 350000);
         assert_eq!(t.cover.as_ref().unwrap().valid_to.to_string(), "2027-01-01");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
