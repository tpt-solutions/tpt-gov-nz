//! Transform layer — maps the raw WINZ legacy format to the department DB schema.
//!
//! This is the only place that knows about both the legacy shape ([`crate::raw`])
//! and the department shape ([`crate::models`]). Swap the source format by writing
//! a new parser that feeds `RawWinzBatch`; the rest of the pipeline is unchanged.

use sqlx::types::Decimal;

use crate::{
    error::IngestError,
    models::{BenefitEntity, CitizenEntity, PaymentEntity, TransformedCitizen},
    raw::RawWinzCitizen,
};

fn dec(v: f64) -> Decimal {
    Decimal::try_from(v).unwrap_or_default()
}

fn parse_date(s: &Option<String>) -> Option<chrono::NaiveDate> {
    s.as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
}

fn parse_date_required(s: &str, field: &str) -> Result<chrono::NaiveDate, IngestError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        IngestError::Transform(format!("invalid {field} date '{s}': {e}"))
    })
}

/// Transform a single raw citizen record into schema-aligned entities.
pub fn transform_citizen(raw: &RawWinzCitizen) -> Result<TransformedCitizen, IngestError> {
    if raw.did.trim().is_empty() {
        return Err(IngestError::Transform(format!(
            "citizen with client_id {} has no DID",
            raw.client_id
        )));
    }

    let citizen = CitizenEntity {
        did: raw.did.clone(),
        client_id: raw.client_id.clone(),
    };

    let mut benefits = Vec::with_capacity(raw.benefits.len());
    for b in &raw.benefits {
        benefits.push(BenefitEntity {
            benefit_type: b.benefit_type.clone(),
            weekly_amount: dec(b.weekly_amount),
            start_date: parse_date(&b.start_date),
            review_date: parse_date(&b.review_date),
            status: b.status.clone(),
        });
    }

    let mut payments = Vec::with_capacity(raw.payments.len());
    for p in &raw.payments {
        payments.push(PaymentEntity {
            benefit_type: p.benefit_type.clone(),
            payment_date: parse_date_required(&p.payment_date, "payment date")?,
            amount: dec(p.amount),
            method: p.method.clone(),
        });
    }

    Ok(TransformedCitizen {
        citizen,
        benefits,
        payments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RawWinzCitizen {
        serde_json::from_value(serde_json::json!({
            "clientId": "WINZ-CLIENT-001",
            "did": "did:gov:nz:test-citizen-001",
            "name": "Alex Tane",
            "benefits": [
                { "benefitType": "jobseeker", "weeklyAmount": 275.40, "startDate": "2024-02-01", "reviewDate": "2026-02-01", "status": "active" }
            ],
            "payments": [
                { "benefitType": "jobseeker", "paymentDate": "2026-07-06", "amount": 275.40, "method": "bank-deposit" }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn maps_citizen_and_benefits() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.citizen.client_id, "WINZ-CLIENT-001");
        assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
        assert_eq!(t.benefits.len(), 1);
        assert_eq!(t.benefits[0].benefit_type, "jobseeker");
        assert_eq!(t.benefits[0].weekly_amount, Decimal::try_from(275.40).unwrap());
    }

    #[test]
    fn maps_payments() {
        let t = transform_citizen(&sample()).unwrap();
        assert_eq!(t.payments.len(), 1);
        assert_eq!(t.payments[0].amount, Decimal::try_from(275.40).unwrap());
    }

    #[test]
    fn rejects_missing_did() {
        let mut raw = sample();
        raw.did = String::new();
        assert!(transform_citizen(&raw).is_err());
    }

    #[test]
    fn rejects_bad_payment_date() {
        let mut raw = sample();
        raw.payments[0].payment_date = "not-a-date".to_owned();
        assert!(transform_citizen(&raw).is_err());
    }
}
