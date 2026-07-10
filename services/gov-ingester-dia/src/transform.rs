//! Transform layer — maps the raw DIA legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        BirthCertEntity, CitizenEntity, CitizenshipEntity, PassportEntity, TransformedCitizen,
    },
    raw::RawDiaCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawDiaCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with passport {} has no DID",
            raw.passport_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        passport_number: raw.passport_number.clone(),
    };

    let passport = raw.passport.as_ref().map(|p| PassportEntity {
        passport_number: raw.passport_number.clone(),
        expiry_date: parse_date(&p.expiry_date, "expiry date")?,
        renewable: p.renewable,
    });

    let birth_cert = raw.birth_cert.as_ref().map(|b| BirthCertEntity {
        certificate_number: b.certificate_number.clone(),
        date_of_birth: parse_date(&b.date_of_birth, "date of birth")?,
        place_of_birth: b.place_of_birth.clone(),
        parents: b.parents.clone(),
    });

    let citizenship = raw.citizenship.as_ref().map(|c| CitizenshipEntity {
        status: c.status.clone(),
        certificate_number: c.certificate_number.clone(),
        granted_at: c
            .granted_at
            .as_deref()
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
    });

    Ok(TransformedCitizen {
        citizen,
        passport,
        birth_cert,
        citizenship,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawDiaCitizen {
        serde_json::from_value(serde_json::json!({
            "passportNumber": "XA123456",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "passport": { "expiryDate": "2028-03-15", "renewable": true },
            "birthCert": { "certificateNumber": "BC-9001", "dateOfBirth": "1990-05-20", "placeOfBirth": "Auckland, New Zealand", "parents": "R. Tane and H. Tane" },
            "citizenship": { "status": "citizen-by-birth" }
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_passport() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.passport_number, "XA123456");
        assert!(t.passport.is_some());
        assert_eq!(t.passport.as_ref().unwrap().expiry_date.to_string(), "2028-03-15");
    }

    #[test]
    fn maps_birth_cert_and_citizenship() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.birth_cert.as_ref().unwrap().certificate_number, "BC-9001");
        assert_eq!(t.citizenship.as_ref().unwrap().status, "citizen-by-birth");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_expiry_date() {
        let mut raw = sample();
        raw.passport.as_mut().unwrap().expiry_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
