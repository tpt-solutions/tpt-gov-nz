//! Transform layer — maps the raw MPI legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        CertificationEntity, CitizenEntity, RegistrationEntity, TransformedCitizen,
    },
    raw::RawMpiCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMpiCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with MPI id {} has no DID",
            raw.mpi_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        mpi_id: raw.mpi_id.clone(),
    };

    let registrations = raw
        .registrations
        .iter()
        .map(|r| {
            Ok(RegistrationEntity {
                nzbn: r.nzbn.clone(),
                business_name: r.business_name.clone(),
                r#type: r.r#type.clone(),
                status: r.status.clone(),
                registered_date: parse_date(&r.registered_date, "registered date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let certifications = raw
        .certifications
        .iter()
        .map(|c| {
            Ok(CertificationEntity {
                cert_number: c.cert_number.clone(),
                category: c.category.clone(),
                issued_date: parse_date(&c.issued_date, "issued date")?,
                expires_date: parse_date(&c.expires_date, "expires date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        registrations,
        certifications,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMpiCitizen {
        serde_json::from_value(serde_json::json!({
            "mpiId": "MPI-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "registrations": [{ "nzbn": "9429046000000", "businessName": "Tane Orchards Ltd", "type": "food-business", "status": "registered", "registeredDate": "2024-03-12" }],
            "certifications": [{ "certNumber": "MPI-CERT-2026-001", "category": "export-certificate", "issuedDate": "2026-01-10", "expiresDate": "2027-01-09" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_registration() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.mpi_id, "MPI-100001");
        assert_eq!(t.registrations.len(), 1);
        assert_eq!(t.registrations[0].nzbn, "9429046000000");
        assert_eq!(t.registrations[0].status, "registered");
    }

    #[test]
    fn maps_certification() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.certifications.len(), 1);
        assert_eq!(t.certifications[0].cert_number, "MPI-CERT-2026-001");
        assert_eq!(t.certifications[0].category, "export-certificate");
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
        raw.registrations[0].registered_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_expires_date() {
        let mut raw = sample();
        raw.certifications[0].expires_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
