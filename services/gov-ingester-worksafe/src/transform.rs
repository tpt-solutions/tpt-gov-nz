//! Transform layer — maps the raw WorkSafe New Zealand legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CitizenEntity, InspectionsEntity, InvestigationsEntity, TransformedCitizen},
    raw::RawWorksafeCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| IngestError::Transform(format!("invalid {field} date '{s}': {e}")))
}

pub fn transform_citizen(raw: &RawWorksafeCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with worksafe_id {} has no DID",
            raw.worksafe_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        worksafe_id: raw.worksafe_id.clone(),
    };

    let inspections = raw
        .inspections
        .iter()
        .map(|c| {
            Ok(InspectionsEntity {
                reference: c.reference.clone(),
                site: c.site.clone(),
                inspection_date: parse_date(&c.inspection_date, "inspection_date")?,
                outcome: c.outcome.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let investigations = raw
        .investigations
        .iter()
        .map(|c| {
            Ok(InvestigationsEntity {
                reference: c.reference.clone(),
                matter: c.matter.clone(),
                status: c.status.clone(),
                opened_date: parse_date(&c.opened_date, "opened_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        inspections,
        investigations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawWorksafeCitizen {
        let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/worksafe_batch.json")).unwrap();
                let batch: crate::raw::RawWorksafeBatch = serde_json::from_slice(&bytes).unwrap();
                batch.citizens.into_iter().next().unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.worksafe_id, "WS-100001");
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.inspections.first().unwrap().reference, "WS-I-2026-003");
        assert_eq!(
            t.inspections.first().unwrap().site,
            "Tane Construction Ltd"
        );
        assert_eq!(
            t.inspections.first().unwrap().inspection_date.to_string(),
            "2026-02-10"
        );
        assert_eq!(
            t.inspections.first().unwrap().outcome,
            "Compliance order issued"
        );

        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(
            t.investigations.first().unwrap().reference,
            "WS-INV-2026-011"
        );
        assert_eq!(
            t.investigations.first().unwrap().matter,
            "Fatality inquiry"
        );
        assert_eq!(t.investigations.first().unwrap().status, "ongoing");
        assert_eq!(
            t.investigations.first().unwrap().opened_date.to_string(),
            "2026-01-15"
        );
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
