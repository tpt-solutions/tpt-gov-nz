//! Unit tests for the MBIE ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-mbie --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawMbieBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mbie_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMbieBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.person_id, "MBIE-P-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.businesses.len(), 1);
    assert_eq!(t.businesses[0].nzbn, "9429000000001");
    assert_eq!(t.directorships.len(), 1);
    assert_eq!(t.directorships[0].nzbn, "9429000000001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.businesses.len(), 1);
    assert_eq!(t2.businesses[0].entity_type, "sole-trader");
    assert_eq!(t2.directorships.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMbieBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "personId": "MBIE-P-9", "did": "", "businessRegistrations": [], "directorships": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_registered_date_is_rejected() {
    let raw: RawMbieBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "personId": "MBIE-P-9", "did": "did:gov:nz:t", "businessRegistrations": [{ "nzbn": "N1", "entityName": "X Ltd", "entityType": "company", "status": "registered", "registeredDate": "nope" }], "directorships": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawMbieBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "personId": "MBIE-P-9", "did": "did:gov:nz:t", "businessRegistrations": [], "directorships": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.businesses.len(), 0);
    assert_eq!(t.directorships.len(), 0);
}
