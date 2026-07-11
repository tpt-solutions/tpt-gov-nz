//! Unit tests for the Ministry of Transport ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawMotBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mot_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMotBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.mot_id, "MOT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.strategies.len(), 1);
    assert_eq!(t.strategies[0].title, "Te Tangi a Te Manu");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.programmes.len(), 1);
    assert_eq!(t.programmes[0].name, "Road maintenance boost");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMotBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "mot_id": "MOT-100001", "did": "", "strategies": [], "programmes": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
