//! Transform layer — maps the raw New Zealand Defence Force legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ServiceRecordsEntity,DeploymentsEntity,CitizenEntity, TransformedCitizen},
    raw::RawNzdfCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawNzdfCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with nzdf_id {} has no DID",
            raw.nzdf_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        nzdf_id: raw.nzdf_id.clone(),
    };

    let service_records = raw
        .service_records
        .iter()
        .map(|c| {
            Ok(ServiceRecordsEntity {
                service_no: c.service_no.clone(),
                branch: c.branch.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let deployments = raw
        .deployments
        .iter()
        .map(|c| {
            Ok(DeploymentsEntity {
                operation: c.operation.clone(),
                country: c.country.clone(),
                year: c.year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        service_records,
        deployments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawNzdfCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.nzdf_id, "NZDF-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.service_records.as_ref().unwrap().service_no, "NZDF-55821");
         assert_eq!(t.service_records.as_ref().unwrap().branch, "Army");
         assert_eq!(t.service_records.as_ref().unwrap().status, "active");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.deployments.as_ref().unwrap().operation, "Burnham readiness");
         assert_eq!(t.deployments.as_ref().unwrap().country, "NZ");
         assert_eq!(t.deployments.as_ref().unwrap().year, 2025);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
