//! Unit tests for the Ministry of Education ingester transform layer (no DB required).

use std::path::PathBuf;

use crate::{raw::RawMoeBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/moe_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawMoeBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.moe_id, "MOE-100001");
    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert!(t.enrolment.is_some());
    assert_eq!(t.enrolment.as_ref().unwrap().school, "Porirua College");

    let t = transform_citizen(&batch.citizens[0]).unwrap();
    assert_eq!(t.student_support.len(), 1);
    assert_eq!(t.student_support[0].service, "Learning support");
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawMoeBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "moe_id": "MOE-100001", "did": "", "enrolment": null, "student_support": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}
