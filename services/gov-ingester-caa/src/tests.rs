//! Unit tests for the Civil Aviation Authority ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawCaaBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/caa_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawCaaBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.caa_id, "CAA-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.licences.len(), 1);
    assert_eq!(t.licences[0].licence_no, "CAA-P-55821");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.aircraft.len(), 1);
    assert_eq!(t.aircraft[0].registration, "ZK-TAN");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawCaaBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "caa_id": "CAA-100001", "did": "", "licences": [], "aircraft": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
