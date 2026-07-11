//! Transform layer — maps the raw Crown Law Office legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{LegalOpinionsEntity,LitigationEntity,CitizenEntity, TransformedCitizen},
    raw::RawCrownlawCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawCrownlawCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with crownlaw_id {} has no DID",
            raw.crownlaw_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        crownlaw_id: raw.crownlaw_id.clone(),
    };

    let legal_opinions = raw
        .legal_opinions
        .iter()
        .map(|c| {
            Ok(LegalOpinionsEntity {
                reference: c.reference.clone(),
                topic: c.topic.clone(),
                issued_date: parse_date(&c.issued_date, "issued_date")?,
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let litigation = raw
        .litigation
        .iter()
        .map(|c| {
            Ok(LitigationEntity {
                case_name: c.case_name.clone(),
                crown_role: c.crown_role.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        legal_opinions,
        litigation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawCrownlawCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.crownlaw_id, "CL-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.legal_opinions.as_ref().unwrap().reference, "CL-OP-2026-001");
         assert_eq!(t.legal_opinions.as_ref().unwrap().topic, "Treaty settlement wording");
         assert_eq!(t.legal_opinions.as_ref().unwrap().issued_date.to_string(), "2026-02-18");
         assert_eq!(t.legal_opinions.as_ref().unwrap().status, "final");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.litigation.as_ref().unwrap().case_name, "Re Crown assets");
         assert_eq!(t.litigation.as_ref().unwrap().crown_role, "Defendant");
         assert_eq!(t.litigation.as_ref().unwrap().status, "ongoing");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
