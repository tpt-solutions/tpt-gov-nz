//! Unit tests for the HUD ingester transform layer (no DB required).
//!
//! Run with:
//!
//! ```sh
//! cargo test -p gov-ingester-hud --lib
//! ```

use std::path::PathBuf;

use crate::{raw::RawHudBatch, transform::transform_citizen};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/hud_batch.json")
}

#[tokio::test]
async fn fixture_parses_and_transforms() {
    let bytes = tokio::fs::read(fixture_path()).await.unwrap();
    let batch: RawHudBatch = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch.citizens.len(), 2);

    let first = &batch.citizens[0];
    assert_eq!(first.client_number, "HUD-100001");
    let t = transform_citizen(first).unwrap();
    assert_eq!(t.citizen.did, "did:gov:nz:test-citizen-001");
    assert_eq!(t.applications.len(), 1);
    assert_eq!(t.applications[0].application_number, "HUD-A5001");
    assert_eq!(t.tenancies.len(), 1);
    assert_eq!(t.tenancies[0].tenancy_id, "HUD-TEN-1");
    assert_eq!(t.maintenance_requests.len(), 1);
    assert_eq!(t.maintenance_requests[0].request_number, "HUD-M3001");

    let second = &batch.citizens[1];
    let t2 = transform_citizen(second).unwrap();
    assert_eq!(t2.citizen.did, "did:gov:nz:test-citizen-002");
    assert_eq!(t2.applications.len(), 1);
    assert_eq!(t2.applications[0].status, "approved");
    assert_eq!(t2.tenancies.len(), 0);
}

#[test]
fn missing_did_is_rejected() {
    let raw: RawHudBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "HUD-9", "did": "", "applications": [], "tenancies": [], "maintenanceRequests": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn bad_submitted_date_is_rejected() {
    let raw: RawHudBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "HUD-9", "did": "did:gov:nz:t", "applications": [{ "applicationNumber": "A1", "applicationType": "public-housing", "status": "submitted", "submittedDate": "nope" }], "tenancies": [], "maintenanceRequests": [] }]
    }))
    .unwrap();
    assert!(transform_citizen(&raw.citizens[0]).is_err());
}

#[test]
fn empty_citizen_is_valid_without_records() {
    let raw: RawHudBatch = serde_json::from_value(serde_json::json!({
        "batchId": "B", "generatedAt": "x", "source": "mock",
        "citizens": [{ "clientNumber": "HUD-9", "did": "did:gov:nz:t", "applications": [], "tenancies": [], "maintenanceRequests": [] }]
    }))
    .unwrap();
    let t = transform_citizen(&raw.citizens[0]).unwrap();
    assert_eq!(t.applications.len(), 0);
    assert_eq!(t.tenancies.len(), 0);
    assert_eq!(t.maintenance_requests.len(), 0);
}
