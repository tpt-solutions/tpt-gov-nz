//! Transform layer — maps the raw New Zealand Defence Force legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, DeploymentsEntity, ServiceRecordsEntity, TransformedCitizen},
    raw::RawNzdfCitizen,
};

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
                year: c.year,
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
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/nzdf_batch.json"
        ))
        .unwrap();
        let batch: crate::raw::RawNzdfBatch = serde_json::from_slice(&bytes).unwrap();
        batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.nzdf_id, "NZDF-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.service_records.first().unwrap().service_no, "NZDF-55821");
        assert_eq!(t.service_records.first().unwrap().branch, "Army");
        assert_eq!(t.service_records.first().unwrap().status, "active");

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.deployments.first().unwrap().operation,
            "Burnham readiness"
        );
        assert_eq!(t.deployments.first().unwrap().country, "NZ");
        assert_eq!(t.deployments.first().unwrap().year, 2025);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
