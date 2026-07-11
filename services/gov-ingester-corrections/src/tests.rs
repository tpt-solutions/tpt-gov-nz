//! Unit tests for the Corrections ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-corrections --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawCorrectionsBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/corrections_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawCorrectionsBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.corrections_id, "COR-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert!(t.probation.is_some());
    assert_eq!(t.probation.as_ref().unwrap().officer_name, "Officer R. Reedy");
    assert_eq!(t.cases.len(), 1);
    assert_eq!(t.cases[0].case_number, "COR-C0001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert!(t2.probation.is_some());
    assert_eq!(t2.cases.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawCorrectionsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "correctionsId": "COR-9", "did": "", "case": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_start_date_is_rejected() {
    let raw: RawCorrectionsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "correctionsId": "COR-9", "did": "did:gov:nz:t", "case": [{ "caseNumber": "C1", "sentenceType": "supervision", "startDate": "nope", "summary": "x" }] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawCorrectionsBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "correctionsId": "COR-9", "did": "did:gov:nz:t", "case": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert!(t.probation.is_none());
    assert_eq!(t.cases.len(), 0);
}
