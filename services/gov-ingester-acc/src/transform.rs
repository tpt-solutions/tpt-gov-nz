//! Transform layer — maps the raw ACC legacy format to the department DB schema.

use crate::{
    error::IngestError,
    models::{
        CitizenEntity, ClaimEntity, EntitlementEntity, RehabilitationEntity, TransformedCitizen,
    },
    raw::RawAccCitizen,
};

fn parse_date(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

pub fn transform_citizen(raw: &RawAccCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with client number {} has no DID",
            raw.client_number
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        client_number: raw.client_number.clone(),
    };

    let claims = raw
        .claims
        .iter()
        .map(|c| ClaimEntity {
            claim_number: c.claim_number.clone(),
            claim_type: c.claim_type.clone(),
            status: c.status.clone(),
            injury_date: parse_date(&c.injury_date, "injury date")?,
            description: c.description.clone(),
            weekly_compensation: c.weekly_compensation,
        })
        .collect::<Vec<_>>();

    let entitlements = raw.entitlements.as_ref().map(|e| EntitlementEntity {
        has_entitlement: e.has_entitlement,
        r#type: e.r#type.clone(),
        weekly_amount: e.weekly_amount,
        remaining_weeks: e.remaining_weeks,
    });

    let rehabilitation = raw
        .rehabilitation
        .iter()
        .map(|r| RehabilitationEntity {
            plan_id: r.plan_id.clone(),
            description: r.description.clone(),
            status: r.status.clone(),
            provider: r.provider.clone(),
            next_review: r
                .next_review
                .as_deref()
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
        })
        .collect::<Vec<_>>();

    Ok(TransformedCitizen {
        citizen,
        claims,
        entitlements,
        rehabilitation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawAccCitizen {
        serde_json::from_value(serde_json::json!({
            "clientNumber": "ACC-100001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "claims": [{ "claimNumber": "ACC-5001", "claimType": "work", "status": "open", "injuryDate": "2025-02-10", "description": "Lower back strain", "weeklyCompensation": 420 }],
            "entitlements": { "hasEntitlement": true, "type": "Weekly compensation", "weeklyAmount": 420, "remainingWeeks": 18 },
            "rehabilitation": [{ "planId": "PLAN-1", "description": "Physio + return-to-work", "status": "active", "provider": "Metro Rehab", "nextReview": "2026-01-15" }]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_claim() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_number, "ACC-100001");
        assert_eq!(t.claims.len(), 1);
        assert_eq!(t.claims[0].claim_number, "ACC-5001");
        assert_eq!(t.claims[0].weekly_compensation, Some(420.0));
    }

    #[test]
    fn maps_entitlement_and_rehab() {
        let t = transform_citizen(&sample()).unwrap();
        let e = t.entitlements.as_ref().unwrap();
        assert!(e.has_entitlement);
        assert_eq!(e.weekly_amount, Some(420.0));
        assert_eq!(t.rehabilitation.len(), 1);
        assert_eq!(t.rehabilitation[0].plan_id, "PLAN-1");
        assert_eq!(
            t.rehabilitation[0].next_review.unwrap().to_string(),
            "2026-01-15"
        );
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_injury_date() {
        let mut raw = sample();
        raw.claims[0].injury_date = "soon".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
