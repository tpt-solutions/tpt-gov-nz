//! Integration + unit tests for the WINZ department service.
//!
//! These use `sqlx::test`, which spins up an ephemeral database (from `DATABASE_URL`),
//! runs the crate migrations (including the dev seed), and hands each test a migrated
//! `PgPool`. Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://winz:winz_dev_password@localhost:5432/winz cargo test -p gov-dept-winz
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, WinzError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

// ── resolve ──────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID)
        .await
        .unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.client_id, "WINZ-CLIENT-001");
}

#[sqlx::test]
async fn resolve_not_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, "did:gov:nz:does-not-exist")
        .await
        .unwrap();
    assert!(row.is_none());
}

#[sqlx::test]
async fn resolve_route_not_found(pool: PgPool) {
    let app = build_app(pool);
    let body = json!({ "did": "did:gov:nz:missing" }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/resolve")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── fetch_data scope combinations ──────────────────────────────────────────────

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
    assert_eq!(data["clientId"], "WINZ-CLIENT-001");
    assert!(data["activeBenefits"].is_null());
    assert!(data["payments"].is_null());
    assert!(data["caseNotes"].is_null());
}

#[sqlx::test]
async fn fetch_data_benefits_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["winz:benefits"]).await;
    let benefits = data["activeBenefits"].as_array().unwrap();
    assert_eq!(benefits.len(), 2);
    let total = data["totalWeeklyPayment"].as_str().unwrap();
    assert_eq!(total, "405.40");
}

#[sqlx::test]
async fn fetch_data_payments_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["winz:payments"]).await;
    let payments = data["payments"].as_array().unwrap();
    assert_eq!(payments.len(), 2);
    assert_eq!(payments[0]["benefitType"], "accommodation-supplement");
}

#[sqlx::test]
async fn fetch_data_case_notes_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["winz:case-notes"]).await;
    let notes = data["caseNotes"].as_array().unwrap();
    assert_eq!(notes.len(), 1);
    assert!(notes[0]["note"].as_str().unwrap().contains("approved"));
}

#[sqlx::test]
async fn fetch_data_all_scopes(pool: PgPool) {
    let data = fetch_data(
        &pool,
        &["winz:benefits", "winz:payments", "winz:case-notes"],
    )
    .await;
    assert!(data["activeBenefits"].is_array());
    assert!(data["payments"].is_array());
    assert!(data["caseNotes"].is_array());
}

// ── actions: valid ─────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_request_appointment_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-appointment",
        &json!({ "reason": "Need to update my bank details" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);

    let notes = crate::db::fetch_case_notes(&pool, citizen_id(&pool).await)
        .await
        .unwrap();
    assert_eq!(notes.len(), 2);
}

#[sqlx::test]
async fn action_request_appointment_invalid_empty_reason(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-appointment",
        &json!({ "reason": "   " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, WinzError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_submit_benefit_review_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "submit-benefit-review",
        &json!({ "notes": "Started part-time work 10 hrs/week" }),
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
    assert!(matches!(err, WinzError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_is_always_audited(pool: PgPool) {
    let _ = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-appointment",
        &json!({ "reason": "Review my entitlements" }),
        "citizen",
        Some("advisory"),
    )
    .await;

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM actions_log WHERE action_type = 'request-appointment'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, Some(1));
}

// ── full HTTP round-trip ───────────────────────────────────────────────────────

#[sqlx::test]
async fn http_round_trip(pool: PgPool) {
    let app = build_app(pool);

    // 1. health
    let resp = app
        .clone()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 2. resolve
    let resolve = Request::builder()
        .method("POST")
        .uri("/citizen/resolve")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "did": TEST_DID }).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(resolve).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["deptLocalId"], "WINZ-CLIENT-001");

    // 3. fetch data with a scope
    let data = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "did": TEST_DID, "scopes": ["winz:benefits"] }).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["activeBenefits"].as_array().unwrap().len(), 2);

    // 4. benefits list
    let benefits = Request::builder()
        .uri(format!("/citizen/{TEST_DID}/benefits"))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(benefits).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["benefits"].as_array().unwrap().len(), 2);
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_ird_requests_winz_benefits_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "winz",
        vec!["winz:benefits".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["winz:benefits"],
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

    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let data: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(data["activeBenefits"].as_array().unwrap().len(), 2);
}

#[sqlx::test]
async fn cross_dept_request_without_grant_is_forbidden(pool: PgPool) {
    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["winz:benefits"],
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
