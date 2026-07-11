//! Transform layer — maps the raw Department of the Prime Minister and Cabinet legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{HonoursEntity,EngagementsEntity,CitizenEntity, TransformedCitizen},
    raw::RawDpmcCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawDpmcCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with dpmc_id {} has no DID",
            raw.dpmc_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        dpmc_id: raw.dpmc_id.clone(),
    };

    let honours = raw
        .honours
        .iter()
        .map(|c| {
            Ok(HonoursEntity {
                award_year: c.award_year.clone(),
                award: c.award.clone(),
                status: c.status.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let engagements = raw
        .engagements
        .iter()
        .map(|c| {
            Ok(EngagementsEntity {
                event_name: c.event_name.clone(),
                event_date: parse_date(&c.event_date, "event_date")?,
                location: c.location.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        honours,
        engagements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawDpmcCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.dpmc_id, "DPMC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.honours.as_ref().unwrap().award_year, 2025);
         assert_eq!(t.honours.as_ref().unwrap().award, "Queen's Service Medal");
         assert_eq!(t.honours.as_ref().unwrap().status, "nominated");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.engagements.as_ref().unwrap().event_name, "Citizens' Honours Reception");
         assert_eq!(t.engagements.as_ref().unwrap().event_date.to_string(), "2026-05-12");
         assert_eq!(t.engagements.as_ref().unwrap().location, "Wellington");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
