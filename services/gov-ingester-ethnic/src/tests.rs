//! Unit tests for the Ministry for Ethnic Communities ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawEthnicBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/ethnic_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawEthnicBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.ethnic_id, "ETH-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.programmes.len(), 1);
    assert_eq!(t.programmes[0].programme_name, "Ethnic Communities Graduate Programme");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.community_grants.len(), 1);
    assert_eq!(t.community_grants[0].grant_name, "Community-led response fund");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawEthnicBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "ethnic_id": "ETH-100001", "did": "", "programmes": [], "community_grants": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
