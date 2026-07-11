//! Unit tests for the TPK ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-tpk --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawTpkBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/tpk_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawTpkBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.tpk_id, "TPK-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.programmes.len(), 1);
    assert_eq!(t.programmes[0].programme_name, "Te Hono");
    assert_eq!(t.funding.len(), 1);
    assert_eq!(t.funding[0].grant_id, "TPK-G1001");
    assert_eq!(t.funding[0].amount, 5000);

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.programmes.len(), 1);
    assert_eq!(t2.programmes[0].region, "Tāmaki Makaurau");
    assert_eq!(t2.funding.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawTpkBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "tpkId": "TPK-9", "did": "", "programmes": [], "funding": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn negative_amount_is_rejected() {
    let raw: RawTpkBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "tpkId": "TPK-9", "did": "did:gov:nz:t", "programmes": [], "funding": [{ "grantId": "G1", "amount": -1, "purpose": "x", "status": "approved" }] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawTpkBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "tpkId": "TPK-9", "did": "did:gov:nz:t", "programmes": [], "funding": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.programmes.len(), 0);
    assert_eq!(t.funding.len(), 0);
}
