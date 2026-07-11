//! Unit tests for the DOC ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-doc --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawDocBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/doc_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawDocBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.doc_id, "DOC-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.permits.len(), 1);
    assert_eq!(t.permits[0].permit_number, "DOC-P1001");
    assert_eq!(t.concessions.len(), 1);
    assert_eq!(t.concessions[0].concession_id, "DOC-C2001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.permits.len(), 0);
    assert_eq!(t2.concessions.len(), 1);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawDocBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "docId": "DOC-9", "did": "", "permits": [], "concessions": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_expires_date_is_rejected() {
    let raw: RawDocBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "docId": "DOC-9", "did": "did:gov:nz:t", "permits": [{ "permitNumber": "P1", "activity": "hunt", "location": "x", "status": "active", "expiresDate": "nope" }], "concessions": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawDocBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "docId": "DOC-9", "did": "did:gov:nz:t", "permits": [], "concessions": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.permits.len(), 0);
    assert_eq!(t.concessions.len(), 0);
}
