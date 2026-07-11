//! Unit tests for the Customs ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-customs --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawCustomsBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/customs_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawCustomsBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.traveller_id, "CUST-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert!(t.travel.is_some());
    assert_eq!(t.travel.as_ref().unwrap().passport_number, "P1234567");
    assert_eq!(t.declarations.len(), 1);
    assert_eq!(t.declarations[0].declaration_id, "CUST-DCL-1");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert!(t2.travel.is_some());
    assert_eq!(t2.declarations.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawCustomsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "travellerId": "CUST-9", "did": "", "declarations": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_travel_date_is_rejected() {
    let raw: RawCustomsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "travellerId": "CUST-9", "did": "did:gov:nz:t", "travel": { "passportNumber": "P1", "lastArrival": "nope", "arrivalPort": "Auckland", "frequentTraveller": false }, "declarations": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawCustomsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "travellerId": "CUST-9", "did": "did:gov:nz:t", "declarations": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert!(t.travel.is_none());
    assert_eq!(t.declarations.len(), 0);
}
