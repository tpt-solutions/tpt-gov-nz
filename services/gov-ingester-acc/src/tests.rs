//! Unit tests for the ACC ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-acc --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawAccBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/acc_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawAccBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.client_number, "ACC-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.claims.len(), 1);
    assert_eq!(t.claims[0].claim_number, "ACC-5001");
    assert_eq!(t.claims[0].weekly_compensation, Some(420.0));
    let ent = t.entitlements.as_ref().unwrap();
    assert!(ent.has_entitlement);
    assert_eq!(ent.weekly_amount, Some(420.0));
    assert_eq!(ent.remaining_weeks, Some(18));
    assert_eq!(t.rehabilitation.len(), 1);
    assert_eq!(t.rehabilitation[0].plan_id, "PLAN-1");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.claims.len(), 1);
    assert_eq!(t2.claims[0].status, "closed");
    assert!(t2.entitlements.is_none());
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawAccBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "ACC-9", "did": "", "claims": [], "rehabilitation": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_injury_date_is_rejected() {
    let raw: RawAccBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "ACC-9", "did": "did:gov:nz:t", "claims": [{ "claimNumber": "C1", "claimType": "work", "status": "open", "injuryDate": "nope", "description": "x" }], "rehabilitation": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawAccBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "ACC-9", "did": "did:gov:nz:t", "claims": [], "rehabilitation": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.claims.len(), 0);
    assert!(t.entitlements.is_none());
    assert_eq!(t.rehabilitation.len(), 0);
}
