//! Unit tests for the MSD ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-msd --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawMsdBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/msd_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMsdBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.client_number, "MSD-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert!(t.studylink.is_some());
    assert_eq!(t.studylink.as_ref().unwrap().loan_balance, Some(28450.75));
    assert_eq!(t.case_events.len(), 2);
    assert_eq!(t.case_events[0].event_id, "MSD-EVT-001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert!(t2.studylink.is_some());
    assert_eq!(t2.case_events.len(), 1);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMsdBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MSD-9", "did": "", "caseHistory": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_event_date_is_rejected() {
    let raw: RawMsdBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MSD-9", "did": "did:gov:nz:t", "caseHistory": [{ "eventId": "E1", "eventDate": "nope", "serviceLine": "x", "summary": "y" }] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawMsdBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "MSD-9", "did": "did:gov:nz:t", "caseHistory": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert!(t.studylink.is_none());
    assert_eq!(t.case_events.len(), 0);
}
