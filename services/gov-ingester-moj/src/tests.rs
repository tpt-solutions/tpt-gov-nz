//! Unit tests for the MOJ ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-moj --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawMojBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/moj_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMojBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.client_number, "MOJ-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.fines.len(), 1);
    assert_eq!(t.fines[0].fine_number, "MOJ-F5001");
    assert_eq!(t.fines[0].amount, 150.0);
    assert_eq!(t.disputes.len(), 1);
    assert_eq!(t.disputes[0].dispute_number, "MOJ-D2001");
    assert_eq!(t.court_records.len(), 1);
    assert_eq!(t.court_records[0].case_number, "MOJ-C3001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.fines.len(), 1);
    assert_eq!(t2.fines[0].status, "paid");
    assert_eq!(t2.disputes.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMojBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MOJ-9", "did": "", "fines": [], "disputes": [], "courtRecords": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_offense_date_is_rejected() {
    let raw: RawMojBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MOJ-9", "did": "did:gov:nz:t", "fines": [{ "fineNumber": "F1", "fineType": "traffic", "status": "unpaid", "amount": 1, "offenseDate": "nope", "dueDate": "2026-01-01", "description": "x" }], "disputes": [], "courtRecords": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawMojBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MOJ-9", "did": "did:gov:nz:t", "fines": [], "disputes": [], "courtRecords": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.fines.len(), 0);
    assert_eq!(t.disputes.len(), 0);
    assert_eq!(t.court_records.len(), 0);
}
