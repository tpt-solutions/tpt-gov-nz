//! Unit tests for the Oranga Tamariki ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawOrangaBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/oranga_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawOrangaBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.oranga_id, "OT-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.care_placements.len(), 1);
    assert_eq!(t.care_placements[0].placement_type, "Whānau placement");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.support_services.len(), 1);
    assert_eq!(t.support_services[0].service, "Intensive support");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawOrangaBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "oranga_id": "OT-100001", "did": "", "care_placements": [], "support_services": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
