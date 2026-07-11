//! Unit tests for the Retirement Commission (Te Ara Ahunga Ora) ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawRetirementBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/retirement_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawRetirementBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.retirement_id, "RET-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.retirement_plan.is_some());
    assert_eq!(t.retirement_plan.as_ref().unwrap().has_plan, true);

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.guidance.len(), 1);
    assert_eq!(t.guidance[0].topic, "KiwiSaver contribution rate");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawRetirementBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "retirement_id": "RET-100001", "did": "", "retirement_plan": null, "guidance": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
