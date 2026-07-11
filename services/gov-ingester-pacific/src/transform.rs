//! Transform layer — maps the raw Ministry for Pacific Peoples legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ProgrammesEntity,LanguageServicesEntity,CitizenEntity, TransformedCitizen},
    raw::RawPacificCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawPacificCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with pacific_id {} has no DID",
            raw.pacific_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        pacific_id: raw.pacific_id.clone(),
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

    let language_services = raw
        .language_services
        .iter()
        .map(|c| {
            Ok(LanguageServicesEntity {
                service: c.service.clone(),
                region: c.region.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        programmes,
        language_services,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawPacificCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.pacific_id, "PAC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.programmes.as_ref().unwrap().programme_name, "Tokelau Language Week");
         assert_eq!(t.programmes.as_ref().unwrap().status, "enrolled");
         assert_eq!(t.programmes.as_ref().unwrap().year, 2026);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.language_services.as_ref().unwrap().service, "Gagana Samoa classes");
         assert_eq!(t.language_services.as_ref().unwrap().region, "Auckland");
         assert_eq!(t.language_services.as_ref().unwrap().status, "available");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
