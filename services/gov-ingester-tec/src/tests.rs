//! Unit tests for the Tertiary Education Commission ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawTecBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/tec_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawTecBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.tec_id, "TEC-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.funding.len(), 1);
    assert_eq!(t.funding[0].provider, "Whitireia");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.courses.len(), 1);
    assert_eq!(t.courses[0].course_name, "New Zealand Certificate in IT");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawTecBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "tec_id": "TEC-100001", "did": "", "funding": [], "courses": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
