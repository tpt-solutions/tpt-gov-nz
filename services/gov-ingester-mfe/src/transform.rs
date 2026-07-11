//! Transform layer — maps the raw Ministry for the Environment legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{EmissionsEntity,ReportsEntity,CitizenEntity, TransformedCitizen},
    raw::RawMfeCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawMfeCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with mfe_id {} has no DID",
            raw.mfe_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        mfe_id: raw.mfe_id.clone(),
    };

    let emissions = raw
        .emissions
        .iter()
        .map(|c| {
            Ok(EmissionsEntity {
                report_year: c.report_year.clone(),
                sector: c.sector.clone(),
                tonnes_co2e: c.tonnes_co2e.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let reports = raw
        .reports
        .iter()
        .map(|c| {
            Ok(ReportsEntity {
                title: c.title.clone(),
                published: parse_date(&c.published, "published")?,
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        emissions,
        reports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawMfeCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.mfe_id, "MFE-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.emissions.as_ref().unwrap().report_year, 2025);
         assert_eq!(t.emissions.as_ref().unwrap().sector, "Transport");
         assert_eq!(t.emissions.as_ref().unwrap().tonnes_co2e, 3200.5);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.reports.as_ref().unwrap().title, "Aotearoa New Zealand's Environment 2026");
         assert_eq!(t.reports.as_ref().unwrap().published.to_string(), "2026-05-01");
         assert_eq!(t.reports.as_ref().unwrap().status, "published");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
