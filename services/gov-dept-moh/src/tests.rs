//! Integration + unit tests for the MOH department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://moh:moh_dev_password@localhost:5432/moh cargo test -p gov-dept-moh
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, MohError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.nhi, "NBA1234");
}

#[sqlx::test]
async fn resolve_not_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, "did:gov:nz:missing").await.unwrap();
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
    assert_eq!(data["nhiNumber"], "NBA1234");
    assert!(data["enrolledGP"].is_null());
}

#[sqlx::test]
async fn fetch_data_gp_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moh:gp"]).await;
    assert_eq!(data["enrolledGP"]["practiceName"], "Pukekohe Family Health");
}

#[sqlx::test]
async fn fetch_data_prescriptions_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moh:prescriptions"]).await;
    let ps = data["activePrescriptions"].as_array().unwrap();
    assert_eq!(ps.len(), 1);
    assert_eq!(ps[0]["medication"], "Atorvastatin");
}

#[sqlx::test]
async fn fetch_data_appointments_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moh:appointments"]).await;
    let as_ = data["upcomingAppointments"].as_array().unwrap();
    assert_eq!(as_.len(), 1);
    assert_eq!(as_[0]["type"], "General check-up");
}

#[sqlx::test]
async fn fetch_data_vaccinations_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["moh:vaccinations"]).await;
    let vs = data["vaccinations"].as_array().unwrap();
    assert_eq!(vs.len(), 2);
    assert!(vs.iter().any(|v| v["vaccine"] == "COVID-19" && v["dueForBooster"] == true));
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_request_repeat_prescription_valid(pool: PgPool) {
    let before = crate::db::fetch_prescriptions(&pool, citizen_id(&pool).await).await.unwrap();
    let pid = before[0].id.to_string();
    let remaining_before = before[0].repeats_remaining;

    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-repeat-prescription",
        &json!({ "prescriptionId": pid }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
    assert_eq!(resp.0["repeatsRemaining"], remaining_before - 1);
}

#[sqlx::test]
async fn action_request_repeat_prescription_invalid_id(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-repeat-prescription",
        &json!({ "prescriptionId": "abc" }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, MohError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_book_appointment_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "book-appointment",
        &json!({ "provider": "Dr. R. Smith", "type": "Blood test", "date": "2026-08-01T09:00:00+12:00" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);

    let as_ = crate::db::fetch_appointments(&pool, citizen_id(&pool).await).await.unwrap();
    assert_eq!(as_.len(), 2);
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
    assert!(matches!(err, MohError::InvalidAction(_)));
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
    assert_eq!(body["deptLocalId"], "NBA1234");

    let data = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "did": TEST_DID, "scopes": ["moh:gp", "moh:vaccinations"] }).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["enrolledGP"]["practiceName"], "Pukekohe Family Health");
    assert_eq!(body["vaccinations"].as_array().unwrap().len(), 2);
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_moh_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "moh",
        vec!["moh:vaccinations".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["moh:vaccinations"],
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
        "scopes": ["moh:vaccinations"],
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
