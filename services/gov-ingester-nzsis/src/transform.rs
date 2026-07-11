//! Transform layer — maps the raw New Zealand Security Intelligence Service legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{MandatesEntity,ThreatsEntity,CitizenEntity, TransformedCitizen},
    raw::RawNzsisCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawNzsisCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with nzsis_id {} has no DID",
            raw.nzsis_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        nzsis_id: raw.nzsis_id.clone(),
    };

    let mandates = raw
        .mandates
        .iter()
        .map(|c| {
            Ok(MandatesEntity {
                reference: c.reference.clone(),
                agency: c.agency.clone(),
                status: c.status.clone(),
                issued_date: parse_date(&c.issued_date, "issued_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let threats = raw
        .threats
        .iter()
        .map(|c| {
            Ok(ThreatsEntity {
                reference: c.reference.clone(),
                category: c.category.clone(),
                status: c.status.clone(),
                assessed_date: parse_date(&c.assessed_date, "assessed_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        mandates,
        threats,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawNzsisCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.nzsis_id, "NZSIS-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.mandates.as_ref().unwrap().reference, "NZSIS-M-2026-002");
         assert_eq!(t.mandates.as_ref().unwrap().agency, "GCSB");
         assert_eq!(t.mandates.as_ref().unwrap().status, "active");
         assert_eq!(t.mandates.as_ref().unwrap().issued_date.to_string(), "2026-01-08");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.threats.as_ref().unwrap().reference, "NZSIS-T-2026-014");
         assert_eq!(t.threats.as_ref().unwrap().category, "Foreign interference");
         assert_eq!(t.threats.as_ref().unwrap().status, "monitored");
         assert_eq!(t.threats.as_ref().unwrap().assessed_date.to_string(), "2026-02-11");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
