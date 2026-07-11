//! Transform layer — maps the raw Customs legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, DeclarationEntity, TravelEntity, TransformedCitizen},
    raw::RawCustomsCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawCustomsCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with traveller id {} has no DID",
            raw.traveller_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        traveller_id: raw.traveller_id.clone(),
    };

    let travel = raw
        .travel
        .as_ref()
        .map(|t| {
            Ok(TravelEntity {
                passport_number: t.passport_number.clone(),
                last_arrival: parse_date(&t.last_arrival, "last arrival")?,
                arrival_port: t.arrival_port.clone(),
                frequent_traveller: t.frequent_traveller,
            })
        })
        .transpose()?;

    let declarations = raw
        .declarations
        .iter()
        .map(|d| {
            Ok(DeclarationEntity {
                declaration_id: d.declaration_id.clone(),
                date: parse_date(&d.date, "declaration date")?,
                country_from: d.country_from.clone(),
                goods_declared: d.goods_declared.clone(),
                status: d.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        travel,
        declarations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawCustomsCitizen {
        serde_json::from_value(serde_json::json!({
            "travellerId": "CUST-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "travel": { "passportNumber": "P1234567", "lastArrival": "2026-06-30", "arrivalPort": "Auckland", "frequentTraveller": true },
            "declarations": [{ "declarationId": "CUST-DCL-1", "date": "2026-06-30", "countryFrom": "Australia", "goodsDeclared": "Personal effects", "status": "submitted" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_travel() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.traveller_id, "CUST-100001");
        assert!(t.travel.is_some());
        let travel = t.travel.unwrap();
        assert_eq!(travel.passport_number, "P1234567");
        assert_eq!(travel.arrival_port, "Auckland");
    }

    #[test]
    fn maps_declarations() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.declarations.len(), 1);
        assert_eq!(t.declarations[0].declaration_id, "CUST-DCL-1");
        assert_eq!(t.declarations[0].country_from, "Australia");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_travel_date() {
        let mut raw = sample();
        raw.travel.as_mut().unwrap().last_arrival = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
