//! Transform layer — maps the raw Government Communications Security Bureau legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{MandatesEntity,EngagementsEntity,CitizenEntity, TransformedCitizen},
    raw::RawGcsbCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawGcsbCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with gcsb_id {} has no DID",
            raw.gcsb_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        gcsb_id: raw.gcsb_id.clone(),
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

    let engagements = raw
        .engagements
        .iter()
        .map(|c| {
            Ok(EngagementsEntity {
                partner: c.partner.clone(),
                engagement_type: c.engagement_type.clone(),
                engagement_date: parse_date(&c.engagement_date, "engagement_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        mandates,
        engagements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawGcsbCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.gcsb_id, "GCSB-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.mandates.as_ref().unwrap().reference, "GCSB-M-2026-001");
         assert_eq!(t.mandates.as_ref().unwrap().agency, "NZSIS");
         assert_eq!(t.mandates.as_ref().unwrap().status, "active");
         assert_eq!(t.mandates.as_ref().unwrap().issued_date.to_string(), "2026-01-05");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.engagements.as_ref().unwrap().partner, "CERT NZ");
         assert_eq!(t.engagements.as_ref().unwrap().engagement_type, "Cyber threat briefing");
         assert_eq!(t.engagements.as_ref().unwrap().engagement_date.to_string(), "2026-02-20");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
