//! Unit tests for the NZQA ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-nzqa --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawNzqaBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/nzqa_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawNzqaBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.nsn, "NSN-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.qualifications.len(), 1);
    assert_eq!(t.qualifications[0].qualification_id, "NZQA-Q1");
    assert_eq!(t.qualifications[0].level, 7);
    assert!(t.transcript.is_some());
    assert_eq!(t.transcript.as_ref().unwrap().total_credits, Some(360));

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.qualifications.len(), 1);
    assert_eq!(t2.qualifications[0].title, "National Certificate in Computing");
    assert!(t2.transcript.is_none());
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawNzqaBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "nsn": "NSN-9", "did": "", "qualifications": [], "transcript": null }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_awarded_date_is_rejected() {
    let raw: RawNzqaBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "nsn": "NSN-9", "did": "did:gov:nz:t", "qualifications": [{ "qualificationId": "Q1", "title": "X", "level": 7, "awardedDate": "nope", "provider": "P" }], "transcript": null }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawNzqaBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "nsn": "NSN-9", "did": "did:gov:nz:t", "qualifications": [], "transcript": null }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.qualifications.len(), 0);
    assert!(t.transcript.is_none());
}
