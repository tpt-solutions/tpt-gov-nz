//! Unit tests for the Te Arawhiti ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawTearawhitiBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/tearawhiti_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawTearawhitiBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.tearawhiti_id, "TAW-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.treaty_settlements.len(), 1);
    assert_eq!(t.treaty_settlements[0].iwi, "Ngāti Toa");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.engagements.len(), 1);
    assert_eq!(t.engagements[0].topic, "Crown engagement hui");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawTearawhitiBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "tearawhiti_id": "TAW-100001", "did": "", "treaty_settlements": [], "engagements": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
