//! Unit tests for the Education Review Office ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawEroBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/ero_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawEroBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.ero_id, "ERO-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.reviews.len(), 1);
    assert_eq!(t.reviews[0].school, "Porirua College");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.reports.len(), 1);
    assert_eq!(t.reports[0].title, "Porirua College annual report");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawEroBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "ero_id": "ERO-100001", "did": "", "reviews": [], "reports": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
