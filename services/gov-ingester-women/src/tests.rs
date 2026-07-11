//! Unit tests for the Ministry for Women ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawWomenBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/women_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawWomenBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.women_id, "WOM-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.programmes.len(), 1);
    assert_eq!(t.programmes[0].programme_name, "Women in Governance");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.insights.len(), 1);
    assert_eq!(t.insights[0].topic, "Pay equity");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawWomenBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "women_id": "WOM-100001", "did": "", "programmes": [], "insights": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
