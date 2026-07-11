//! Transform layer — maps the raw Serious Fraud Office legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{InvestigationsEntity,OutcomesEntity,CitizenEntity, TransformedCitizen},
    raw::RawSfoCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawSfoCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with sfo_id {} has no DID",
            raw.sfo_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        sfo_id: raw.sfo_id.clone(),
    };

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

    let outcomes = raw
        .outcomes
        .iter()
        .map(|c| {
            Ok(OutcomesEntity {
                reference: c.reference.clone(),
                result: c.result.clone(),
                result_date: parse_date(&c.result_date, "result_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        investigations,
        outcomes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawSfoCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.sfo_id, "SFO-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.investigations.as_ref().unwrap().reference, "SFO-2026-014");
         assert_eq!(t.investigations.as_ref().unwrap().matter, "Complex investment fraud");
         assert_eq!(t.investigations.as_ref().unwrap().status, "under-investigation");
         assert_eq!(t.investigations.as_ref().unwrap().opened_date.to_string(), "2026-01-22");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.outcomes.as_ref().unwrap().reference, "SFO-2025-009");
         assert_eq!(t.outcomes.as_ref().unwrap().result, "Prosecution commenced");
         assert_eq!(t.outcomes.as_ref().unwrap().result_date.to_string(), "2025-11-03");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
