//! Integration + unit tests for the NZTA department service.
//!
//! Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://nzta:nzta_dev_password@localhost:5432/nzta cargo test -p gov-dept-nzta
//! ```

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use crate::{build_app, NztaError};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.driver_licence_number, "NZ1234567");
}

#[sqlx::test]
async fn resolve_not_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, "did:gov:nz:missing").await.unwrap();
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
    assert_eq!(data["driverLicenceNumber"], "NZ1234567");
    assert!(data["driverLicence"].is_null());
    assert!(data["vehicles"].is_null());
    assert!(data["ruc"].is_null());
}

#[sqlx::test]
async fn fetch_data_driver_licence_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["nzta:driver-licence"]).await;
    assert_eq!(data["driverLicence"]["licenceNumber"], "NZ1234567");
    assert_eq!(data["driverLicence"]["fullName"], "Alex Tane");
    assert_eq!(data["driverLicence"]["licenceClass"], "1 (car)");
    assert_eq!(data["driverLicence"]["expiryDate"], "2028-09-30");
}

#[sqlx::test]
async fn fetch_data_vehicles_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["nzta:vehicles"]).await;
    let vehicles = data["vehicles"].as_array().unwrap();
    assert_eq!(vehicles.len(), 1);
    assert_eq!(vehicles[0]["registration"], "ABC123");
    assert_eq!(vehicles[0]["make"], "Toyota");
    assert_eq!(vehicles[0]["year"], 2021);
}

#[sqlx::test]
async fn fetch_data_ruc_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["nzta:ruc"]).await;
    let ruc = data["ruc"].as_array().unwrap();
    assert_eq!(ruc.len(), 1);
    assert_eq!(ruc[0]["vehicleRego"], "ABC123");
    assert_eq!(ruc[0]["unitsRemaining"], 1500);
}

// ── actions ──────────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_renew_vehicle_registration_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "renew-vehicle-registration",
        &json!({ "registration": "ABC123", "months": 12 }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_renew_vehicle_registration_invalid_months(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "renew-vehicle-registration",
        &json!({ "registration": "ABC123", "months": 99 }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, NztaError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_request_licence_replacement_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-licence-replacement",
        &json!({ "reason": "Lost in the wash" }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_request_licence_replacement_empty_reason(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-licence-replacement",
        &json!({ "reason": "   " }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, NztaError::InvalidAction(_)));
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
    assert!(matches!(err, NztaError::InvalidAction(_)));
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
        .body(Body::from(json!({ "did": TEST_DID, "scopes": ["nzta:driver-licence", "nzta:vehicles"] }).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["driverLicence"]["licenceNumber"], "NZ1234567");
    assert_eq!(body["vehicles"][0]["registration"], "ABC123");
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::{CredentialIssuer, GovDid};

#[sqlx::test]
async fn cross_dept_requests_nzta_with_valid_grant(pool: PgPool) {
    let issuer = CredentialIssuer::generate();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "ird",
        "nzta",
        vec!["nzta:driver-licence".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["nzta:driver-licence"],
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
        "scopes": ["nzta:driver-licence"],
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
