//! Unit tests for the LINZ ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-linz --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawLinzBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/linz_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawLinzBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.customer_id, "LINZ-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.titles.len(), 1);
    assert_eq!(t.titles[0].title_number, "LNZ-T-1");
    assert_eq!(t.ownership.len(), 1);
    assert_eq!(t.ownership[0].title_number, "LNZ-T-1");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.titles.len(), 1);
    assert_eq!(t2.ownership[0].ownership_share, "1/2");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawLinzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "customerId": "LINZ-9", "did": "", "titles": [], "ownership": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawLinzBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "customerId": "LINZ-9", "did": "did:gov:nz:t", "titles": [], "ownership": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.titles.len(), 0);
    assert_eq!(t.ownership.len(), 0);
}
