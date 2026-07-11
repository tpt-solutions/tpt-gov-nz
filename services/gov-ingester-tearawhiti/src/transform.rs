//! Transform layer — maps the raw Te Arawhiti legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{TreatySettlementsEntity,EngagementsEntity,CitizenEntity, TransformedCitizen},
    raw::RawTearawhitiCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawTearawhitiCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with tearawhiti_id {} has no DID",
            raw.tearawhiti_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        tearawhiti_id: raw.tearawhiti_id.clone(),
    };

    let treaty_settlements = raw
        .treaty_settlements
        .iter()
        .map(|c| {
            Ok(TreatySettlementsEntity {
                iwi: c.iwi.clone(),
                status: c.status.clone(),
                settled_date: parse_date(&c.settled_date, "settled_date")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let engagements = raw
        .engagements
        .iter()
        .map(|c| {
            Ok(EngagementsEntity {
                topic: c.topic.clone(),
                engagement_date: parse_date(&c.engagement_date, "engagement_date")?,
                outcome: c.outcome.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        treaty_settlements,
        engagements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawTearawhitiCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.tearawhiti_id, "TAW-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.treaty_settlements.as_ref().unwrap().iwi, "Ngāti Toa");
         assert_eq!(t.treaty_settlements.as_ref().unwrap().status, "settled");
         assert_eq!(t.treaty_settlements.as_ref().unwrap().settled_date.to_string(), "2024-07-01");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.engagements.as_ref().unwrap().topic, "Crown engagement hui");
         assert_eq!(t.engagements.as_ref().unwrap().engagement_date.to_string(), "2026-04-15");
         assert_eq!(t.engagements.as_ref().unwrap().outcome, "Recommendation agreed");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
