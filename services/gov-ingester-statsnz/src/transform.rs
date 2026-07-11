//! Transform layer — maps the raw Statistics New Zealand legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{CensusEntity, CitizenEntity, ProfileEntity, TransformedCitizen},
    raw::RawStatsnzCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawStatsnzCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with stats id {} has no DID",
            raw.stats_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        stats_id: raw.stats_id.clone(),
    };

    let census = raw
        .census
        .iter()
        .map(|c| {
            Ok(CensusEntity {
                census_year: c.census_year,
                dwelling_type: c.dwelling_type.clone(),
                household_size: c.household_size,
                region: c.region.clone(),
            })
        })
        .collect::<Result<Vec<_>, IngestError>>()?;

    let profile = match &raw.profile {
        Some(p) => Some(ProfileEntity {
            data_summary: p.data_summary.clone(),
            record_count: p.record_count,
            last_updated: parse_date(&p.last_updated, "last updated date")?,
        }),
        None => None,
    };

    Ok(TransformedCitizen {
        citizen,
        census,
        profile,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawStatsnzCitizen {
        serde_json::from_value(serde_json::json!({
            "statsId": "STATS-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "census": [{ "censusYear": 2023, "dwellingType": "house", "householdSize": 4, "region": "Auckland" }],
            "profile": { "dataSummary": "2023 Census response recorded.", "recordCount": 1, "lastUpdated": "2026-06-01" }
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_census() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.stats_id, "STATS-100001");
        assert_eq!(t.census.len(), 1);
        assert_eq!(t.census[0].census_year, 2023);
        assert_eq!(t.census[0].region, "Auckland");
    }

    #[test]
    fn maps_profile() {
        let t = transform_citizen(&sample()).unwrap();
        let p = t.profile.expect("profile present");
        assert_eq!(p.data_summary, "2023 Census response recorded.");
        assert_eq!(p.record_count, 1);
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_profile_date() {
        let mut raw = sample();
        raw.profile = Some(serde_json::from_value(serde_json::json!({
            "dataSummary": "x", "recordCount": 0, "lastUpdated": "soon"
        })).unwrap());
        assert!(transform_citizen(&raw).is_err());
    }
}
