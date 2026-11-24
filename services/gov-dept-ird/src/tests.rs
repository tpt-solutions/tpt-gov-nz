//! Integration + unit tests for the IRD department service.
//!
//! These use `sqlx::test`, which spins up an ephemeral database (from `DATABASE_URL`),
//! runs the crate migrations (including the dev seed), and hands each test a migrated
//! `PgPool`. Run with a Postgres available:
//!
//! ```sh
//! DATABASE_URL=postgresql://ird:ird_dev_password@localhost:5432/ird cargo test -p gov-dept-ird
//! ```

use crate::{IrdError, build_app};
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

const TEST_DID: &str = "did:gov:nz:test-citizen-001";

// ── resolve ──────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn resolve_found(pool: PgPool) {
    let row = crate::db::resolve_by_did(&pool, TEST_DID).await.unwrap();
    let row = row.expect("seed migration should create the test citizen");
    assert_eq!(row.did, TEST_DID);
    assert_eq!(row.ird_number, "123-456-789");
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
    assert_eq!(data["irdNumber"], "123-456-789");
    assert!(data["currentTaxYear"].is_null());
    assert!(data["taxHistory"].is_null());
    assert!(data["gstPeriods"].is_null());
    assert!(data["kiwiSaver"].is_null());
    assert!(data["workingForFamilies"].is_null());
}

#[sqlx::test]
async fn fetch_data_income_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["ird:income"]).await;
    assert!(data["currentTaxYear"].is_object());
    assert_eq!(data["currentTaxYear"]["totalIncome"], "66200.00");
    assert_eq!(data["currentTaxYear"]["employmentIncome"], "65000.00");
    assert_eq!(data["currentTaxYear"]["otherIncome"], "1200.00");
    // tax-summary not granted → no history
    assert!(data["taxHistory"].is_null());
}

#[sqlx::test]
async fn fetch_data_tax_summary_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["ird:tax-summary"]).await;
    assert!(data["currentTaxYear"].is_object());
    assert!(data["taxHistory"].is_array());
    assert_eq!(data["taxHistory"].as_array().unwrap().len(), 1);
}

#[sqlx::test]
async fn fetch_data_gst_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["ird:gst"]).await;
    // Citizen is not GST registered in seed data; the field is present but false.
    assert_eq!(data["gstRegistered"], false);
    assert!(data["gstPeriods"].is_array());
}

#[sqlx::test]
async fn fetch_data_kiwisaver_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["ird:kiwisaver"]).await;
    assert!(data["kiwiSaver"].is_object());
    assert_eq!(data["kiwiSaver"]["membershipStatus"], "active");
    assert_eq!(data["kiwiSaver"]["contributionRate"], "3.00");
    assert_eq!(data["kiwiSaver"]["totalBalance"], "18500.00");
}

#[sqlx::test]
async fn fetch_data_wff_scope(pool: PgPool) {
    let data = fetch_data(&pool, &["ird:wff"]).await;
    let wff = &data["workingForFamilies"];
    assert_eq!(wff["eligible"], true);
    assert_eq!(wff["numberOfDependantChildren"], 2);
    assert!(wff["currentEntitlement"].is_object());
    assert_eq!(wff["currentEntitlement"]["familyTaxCredit"], "127.00");
}

#[sqlx::test]
async fn fetch_data_all_scopes(pool: PgPool) {
    let data = fetch_data(
        &pool,
        &[
            "ird:income",
            "ird:tax-summary",
            "ird:gst",
            "ird:kiwisaver",
            "ird:wff",
        ],
    )
    .await;
    assert!(data["currentTaxYear"].is_object());
    assert!(data["taxHistory"].is_array());
    assert!(data["kiwiSaver"].is_object());
    assert!(data["workingForFamilies"].is_object());
}

// ── actions: valid ─────────────────────────────────────────────────────────────

#[sqlx::test]
async fn action_update_kiwisaver_rate_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "update-kiwisaver-rate",
        &json!({ "newRate": 6.0 }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);

    let ks = crate::db::fetch_kiwisaver(&pool, citizen_id(&pool).await)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(ks.contribution_rate, sqlx::types::Decimal::from(6));
}

#[sqlx::test]
async fn action_request_tax_summary_valid(pool: PgPool) {
    let resp = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "request-tax-summary",
        &json!({}),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
}

#[sqlx::test]
async fn action_update_kiwisaver_rate_invalid(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "update-kiwisaver-rate",
        &json!({ "newRate": 5.0 }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, IrdError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_file_gst_return_invalid_missing_fields(pool: PgPool) {
    let err = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "file-gst-return",
        &json!({ "periodId": "not-a-uuid" }),
        "citizen",
        None,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, IrdError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_file_gst_return_valid(pool: PgPool) {
    // Seed a GST period in a filable state so the action has something to update.
    let cid = citizen_id(&pool).await;
    let pid = sqlx::query_scalar!(
        r#"INSERT INTO gst_periods (citizen_id, period_start, period_end, filing_due, status)
           VALUES ($1, '2025-01-01', '2025-02-28', '2025-03-28', 'due')
           RETURNING id"#,
        cid
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = crate::actions::execute(
        &pool,
        cid,
        "file-gst-return",
        &json!({
            "periodId": pid.to_string(),
            "salesIncome": 1000.0,
            "gstOnSales": 150.0,
            "gstOnPurchases": 50.0,
        }),
        "citizen",
        None,
    )
    .await
    .unwrap();
    assert_eq!(resp.0["success"], true);
    assert_eq!(resp.0["refundOrPayment"], -100.0);

    let period = sqlx::query!(
        "SELECT status, refund_or_payment FROM gst_periods WHERE id = $1",
        pid
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(period.status, "filed");
    assert_eq!(
        period.refund_or_payment,
        Some(sqlx::types::Decimal::from(-100))
    );
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
    assert!(matches!(err, IrdError::InvalidAction(_)));
}

#[sqlx::test]
async fn action_is_always_audited(pool: PgPool) {
    let _ = crate::actions::execute(
        &pool,
        citizen_id(&pool).await,
        "update-kiwisaver-rate",
        &json!({ "newRate": 8.0 }),
        "citizen",
        Some("advisory"),
    )
    .await;

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM actions_log WHERE action_type = 'update-kiwisaver-rate'"
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
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
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
    assert_eq!(body["deptLocalId"], "123-456-789");

    // 3. fetch data with a scope
    let data = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "did": TEST_DID, "scopes": ["ird:kiwisaver"] }).to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["kiwiSaver"]["membershipStatus"], "active");

    // 4. tax-years list
    let years = Request::builder()
        .uri(format!("/citizen/{TEST_DID}/tax-years"))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(years).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["assessmentYears"].as_array().unwrap().len(), 1);
}

// ── cross-department federation (consent-gated) ─────────────────────────────────

use gov_identity_core::GovDid;

#[sqlx::test]
async fn cross_dept_winz_requests_ird_income_with_valid_grant(pool: PgPool) {
    let issuer = gov_test_support::shared_issuer();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "winz",
        "ird",
        vec!["ird:income".into()],
        3600,
    );

    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["ird:income"],
        "requesting_dept_id": "winz",
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
    assert!(data["currentTaxYear"].is_object());
    assert_eq!(data["currentTaxYear"]["totalIncome"], "66200.00");
}

#[sqlx::test]
async fn cross_dept_request_without_grant_is_forbidden(pool: PgPool) {
    let app = build_app(pool);
    let body = json!({
        "did": TEST_DID,
        "scopes": ["ird:income"],
        "requesting_dept_id": "winz",
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

#[sqlx::test]
async fn cross_dept_request_for_scope_not_in_grant_is_forbidden(pool: PgPool) {
    let issuer = gov_test_support::shared_issuer();
    let grant = issuer.issue_data_grant(
        GovDid::parse(TEST_DID).unwrap(),
        "winz",
        "ird",
        vec!["ird:income".into()],
        3600,
    );

    let app = build_app(pool);
    // WINZ asks for kiwisaver, which the income-only grant does not cover.
    let body = json!({
        "did": TEST_DID,
        "scopes": ["ird:kiwisaver"],
        "requesting_dept_id": "winz",
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

// ── read audit ─────────────────────────────────────────────────────────────────

#[sqlx::test]
async fn direct_read_writes_granted_audit_row(pool: PgPool) {
    let _ = fetch_data(&pool, &["ird:kiwisaver"]).await;

    let rows: Vec<(String, bool, Vec<String>)> =
        sqlx::query_as("SELECT outcome, direct_access, scopes FROM data_access_log ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(rows.len(), 1, "one audit row per read");
    assert_eq!(rows[0].0, "granted");
    assert!(rows[0].1, "portal reads are direct access");
    assert_eq!(rows[0].2, vec!["ird:kiwisaver".to_string()]);
}

#[sqlx::test]
async fn denied_cross_department_read_is_audited(pool: PgPool) {
    let app = build_app(pool.clone());
    let body = json!({
        "did": TEST_DID,
        "scopes": ["ird:income"],
        "requesting_dept_id": "winz",
    })
    .to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_ne!(
        resp.status(),
        StatusCode::OK,
        "ungranted cross-department read must be refused"
    );

    let outcomes: Vec<(String, bool)> =
        sqlx::query_as("SELECT outcome, direct_access FROM data_access_log ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(outcomes, vec![("denied".to_string(), false)]);
}

#[sqlx::test]
async fn audit_log_rejects_updates(pool: PgPool) {
    let _ = fetch_data(&pool, &["ird:kiwisaver"]).await;
    let result = sqlx::query("UPDATE data_access_log SET outcome = 'granted'")
        .execute(&pool)
        .await;
    assert!(result.is_err(), "data_access_log is append-only");
}

// ── citizen self-access via the gateway ─────────────────────────────────────────

async fn citizen_self_request(pool: &PgPool, subject: &str) -> axum::http::Response<Body> {
    let app = build_app(pool.clone());
    let req = Request::builder()
        .method("POST")
        .uri("/citizen/data")
        .header("content-type", "application/json")
        .header("x-requesting-dept-id", "citizen")
        .header("x-citizen-did", subject)
        .body(Body::from(
            json!({ "did": TEST_DID, "scopes": ["ird:kiwisaver"] }).to_string(),
        ))
        .unwrap();
    app.oneshot(req).await.unwrap()
}

#[sqlx::test]
async fn citizen_reads_own_record_through_gateway(pool: PgPool) {
    let resp = citizen_self_request(&pool, TEST_DID).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn citizen_cannot_read_another_citizens_record(pool: PgPool) {
    let resp = citizen_self_request(&pool, "did:gov:nz:someone-else").await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let outcome: (String,) = sqlx::query_as("SELECT outcome FROM data_access_log ORDER BY id DESC LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(outcome.0, "denied", "the refused attempt is still audited");
}
