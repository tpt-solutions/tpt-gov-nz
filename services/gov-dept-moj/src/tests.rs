//! Integration + unit tests for the MOJ department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://moj:moj_dev_password@localhost:5432/moj cargo test -p gov-dept-moj
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, MojError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.client_number, "MOJ-100001");
}

#[sqlx::test]
async fn resolve_not_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, "did:gov:nz:missing")
        .await
        .unwrap();
    assert!(row.is_none());
}

async fn fetch_data(pool: &PgPool, scopes: &[&str]) -> Value {
    let app = build_app(pool.clone());
    let body = json!({ "did": TEST_DID, "scopes": scopes }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[sqlx::test]
async fn fetch_data_no_scopes_returns_minimal(pool: PgPool) {
    let data = fetch_data(&pool, &[]).await;
    assert_eq!(data["clientNumber"], "MOJ-100001");
    assert!(data["fines"].is_null());
}

#[sqlx::test]
async fn fetch_data_fines_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moj:fines"]).await;
    let fines = data["fines"].as_array().unwrap();
    assert_eq!(fines.len(), 1);
    assert_eq!(fines[0]["fineNumber"], "MOJ-F5001");
    assert_eq!(fines[0]["fineType"], "traffic");
    assert_eq!(fines[0]["status"], "unpaid");
    assert_eq!(fines[0]["amount"], 150);
}

#[sqlx::test]
async fn fetch_data_disputes_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moj:disputes"]).await;
    let disputes = data["disputes"].as_array().unwrap();
    assert_eq!(disputes.len(), 1);
    assert_eq!(disputes[0]["disputeNumber"], "MOJ-D2001");
    assert_eq!(disputes[0]["claimType"], "tenancy");
}

#[sqlx::test]
async fn fetch_data_court_records_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moj:court-records"]).await;
    let records = data["courtRecords"].as_array().unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["caseNumber"], "MOJ-C3001");
    assert_eq!(records[0]["status"], "open");
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_pay_fine_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "pay-fine",
        &json!({ "fineNumber": "MOJ-F5001" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_pay_fine_rejects_missing_number(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "pay-fine",
        &json!({}),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MojError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_file_dispute_claim_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "file-dispute-claim",
        &json!({ "claimType": "consumer", "description": "Faulty goods" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_file_dispute_claim_rejects_bad_type(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "file-dispute-claim",
        &json!({ "claimType": "rocket", "description": "x" }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MojError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_request_name_change_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-name-change",
        &json!({ "newName": "Alex Rangi", "reason": "Marriage" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_unknown_is_invalid(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "frobnicate",
        &json!({}),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MojError::InvalidAction(_)));
}

// ── full HTTP round-trip ─────────────────────────────────────────────────────────

#[sqlx::test]
async fn http_round_trip(pool: PgPool) {
    let app = build_app(pool);

    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let data = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "did": TEST_DID, "scopes": ["moj:fines", "moj:disputes", "moj:court-records"] })
                .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["clientNumber"], "MOJ-100001");
    assert_eq!(body["fines"][0]["fineNumber"], "MOJ-F5001");
    assert_eq!(body["disputes"][0]["disputeNumber"], "MOJ-D2001");
    assert_eq!(body["courtRecords"][0]["caseNumber"], "MOJ-C3001");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_moj_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "moj",
        vec!["moj:fines".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["moj:fines"],
        "requesting_dept_id": "ird",
        "consent_grants": [grant],
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn cross_dept_request_without_grant_is_forbidden(pool: PgPool) {
    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["moj:fines"],
        "requesting_dept_id": "ird",
        "consent_grants": [],
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ── helpers ────────────────────────────────────────────────────────────────────

async fn citizen_id(pool: &PgPool) -> uuid::Uuid {
    crate::db::resolve_by_did(pool, TEST_DID)
        .await
        .unwrap()
        .expect("test citizen seeded")
        .id
}
