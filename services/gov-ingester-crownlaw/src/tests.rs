//! Unit tests for the Crown Law Office ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawCrownlawBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/crownlaw_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawCrownlawBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.crownlaw_id, "CL-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.legal_opinions.len(), 1);
    assert_eq!(t.legal_opinions[0].reference, "CL-OP-2026-001");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.litigation.len(), 1);
    assert_eq!(t.litigation[0].case_name, "Re Crown assets");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawCrownlawBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "crownlaw_id": "CL-100001", "did": "", "legal_opinions": [], "litigation": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
