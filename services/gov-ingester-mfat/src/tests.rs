//! Unit tests for the Ministry of Foreign Affairs and Trade ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawMfatBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/mfat_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMfatBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.mfat_id, "MFAT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.overseas_missions.len(), 1);
    assert_eq!(t.overseas_missions[0].country, "Australia");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.travel_advisories.len(), 1);
    assert_eq!(t.travel_advisories[0].country, "Indonesia");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMfatBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "mfat_id": "MFAT-100001", "did": "", "overseas_missions": [], "travel_advisories": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
