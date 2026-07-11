//! Unit tests for the Police ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-police --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawPoliceBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/police_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawPoliceBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.client_number, "POL-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.infringements.len(), 1);
    assert_eq!(t.infringements[0].ticket_number, "POL-T5001");
    assert_eq!(t.infringements[0].demerit_points, Some(20));
    assert_eq!(t.reports.len(), 1);
    assert_eq!(t.reports[0].report_number, "POL-R2001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.infringements.len(), 1);
    assert_eq!(t2.infringements[0].status, "paid");
    assert_eq!(t2.reports.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawPoliceBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "POL-9", "did": "", "infringements": [], "reports": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_issue_date_is_rejected() {
    let raw: RawPoliceBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "POL-9", "did": "did:gov:nz:t", "infringements": [{ "ticketNumber": "T1", "offenseType": "speeding", "status": "unpaid", "amount": 1, "issueDate": "nope", "description": "x" }], "reports": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawPoliceBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "POL-9", "did": "did:gov:nz:t", "infringements": [], "reports": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.infringements.len(), 0);
    assert_eq!(t.reports.len(), 0);
}
