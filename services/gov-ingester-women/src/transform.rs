//! Transform layer — maps the raw Ministry for Women legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{ProgrammesEntity,InsightsEntity,CitizenEntity, TransformedCitizen},
    raw::RawWomenCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawWomenCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with women_id {} has no DID",
            raw.women_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        women_id: raw.women_id.clone(),
    };

    let programmes = raw
        .programmes
        .iter()
        .map(|c| {
            Ok(ProgrammesEntity {
                programme_name: c.programme_name.clone(),
                status: c.status.clone(),
                year: c.year.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let insights = raw
        .insights
        .iter()
        .map(|c| {
            Ok(InsightsEntity {
                topic: c.topic.clone(),
                summary: c.summary.clone(),
                published: parse_date(&c.published, "published")?,
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    Ok(TransformedCitizen {
        citizen,
        programmes,
        insights,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawWomenCitizen {
        serde_json::from_value(serde_json::json!([object Object]))
            .unwrap()
    }

    #[test]
    fn maps_citizen_and_entities() {
        let raw = sample();
        let t = transform_citizen(&raw).unwrap();
        assert_eq!(t.citizen.women_id, "WOM-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.programmes.as_ref().unwrap().programme_name, "Women in Governance");
         assert_eq!(t.programmes.as_ref().unwrap().status, "enrolled");
         assert_eq!(t.programmes.as_ref().unwrap().year, 2026);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
         assert_eq!(t.insights.as_ref().unwrap().topic, "Pay equity");
         assert_eq!(t.insights.as_ref().unwrap().summary, "Progress on gender pay gap reporting.");
         assert_eq!(t.insights.as_ref().unwrap().published.to_string(), "2026-03-08");
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }
}
