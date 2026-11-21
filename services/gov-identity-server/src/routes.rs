use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::state::AppState;

// ── Errors ────────────────────────────────────────────────────────────────────

/// A JSON error response returned by the identity server.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    fn internal(error: impl std::fmt::Display) -> Self {
        tracing::error!(error = %error, "identity-server internal error");
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "error": self.status.canonical_reason().unwrap_or("error").to_lowercase(),
                "message": self.message,
            })),
        )
            .into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

// ── Health ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub issuer_did: String,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "gov-identity-server",
        issuer_did: state.issuer.did.to_string(),
    })
}

// ── Issuer public key ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct IssuerKeyResponse {
    pub did: String,
    pub public_key_b64: String,
}

/// Public endpoint: lets departments pin the identity server's verification key.
pub async fn issuer_public_key(State(state): State<AppState>) -> Json<IssuerKeyResponse> {
    Json(IssuerKeyResponse {
        did: state.issuer.did.to_string(),
        public_key_b64: state.issuer.public_key_b64(),
    })
}

// ── Validation helpers ────────────────────────────────────────────────────────

const MAX_DEPT_ID_LEN: usize = 32;
const MAX_SCOPE_LEN: usize = 96;
const MAX_SCOPES: usize = 32;
/// Minimum grant lifetime: 60 seconds. Maximum: 366 days.
const MIN_GRANT_TTL: u64 = 60;
const MAX_GRANT_TTL: u64 = 366 * 24 * 60 * 60;

fn valid_dept_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_DEPT_ID_LEN
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn valid_scope(scope: &str) -> bool {
    if scope.is_empty() || scope.len() > MAX_SCOPE_LEN {
        return false;
    }
    let Some((dept, action)) = scope.split_once(':') else {
        return false;
    };
    valid_dept_id(dept)
        && !action.is_empty()
        && action
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Decode a base64 Ed25519 public key, accepting both standard and URL-safe
/// (padded or unpadded) encodings — the browser wallet emits URL-safe base64.
fn decode_public_key(encoded: &str) -> Result<ed25519_dalek::VerifyingKey, ApiError> {
    use base64::Engine;

    let trimmed = encoded.trim();
    let engines = [
        base64::engine::general_purpose::STANDARD,
        base64::engine::general_purpose::URL_SAFE_NO_PAD,
        base64::engine::general_purpose::URL_SAFE,
    ];
    let bytes = engines
        .iter()
        .find_map(|engine| engine.decode(trimmed).ok())
        .ok_or_else(|| ApiError::bad_request("public_key_b64 is not valid base64"))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| ApiError::bad_request("public key must be 32 bytes"))?;
    ed25519_dalek::VerifyingKey::from_bytes(&arr)
        .map_err(|_| ApiError::bad_request("public key is not a valid Ed25519 key"))
}

fn validate_did(did: &str) -> ApiResult<gov_identity_core::GovDid> {
    gov_identity_core::GovDid::parse(did).map_err(|_| ApiError::bad_request("invalid citizen DID"))
}

// ── DID registration ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterDidRequest {
    pub did: String,
    pub public_key_b64: String,
}

#[derive(Serialize)]
pub struct RegisterDidResponse {
    pub did: String,
    pub created_at: String,
}

#[derive(sqlx::FromRow)]
struct DidRow {
    did: String,
    public_key_b64: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Register a citizen DID and its public key.
///
/// Registration is **first-write-wins**: the key can never be overwritten. A
/// repeat registration with the same key is idempotent; a different key is
/// rejected with `409 Conflict`.
pub async fn register_did(
    State(state): State<AppState>,
    Json(req): Json<RegisterDidRequest>,
) -> ApiResult<(StatusCode, Json<RegisterDidResponse>)> {
    let did = validate_did(&req.did)?;
    let verifying_key = decode_public_key(&req.public_key_b64)?;

    // The DID is derived from the public key, so this binds the key to the DID
    // and prevents a caller registering a DID they do not control.
    let expected = gov_identity_core::GovDid::from_verifying_key(&verifying_key);
    if expected.as_str() != did.as_str() {
        return Err(ApiError::bad_request(
            "public key does not match DID fingerprint",
        ));
    }

    let inserted = sqlx::query_as::<_, DidRow>(
        r#"INSERT INTO did_documents (did, public_key_b64)
           VALUES ($1, $2)
           ON CONFLICT (did) DO NOTHING
           RETURNING did, public_key_b64, created_at"#,
    )
    .bind(did.as_str())
    .bind(req.public_key_b64.trim())
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    if let Some(row) = inserted {
        return Ok((
            StatusCode::CREATED,
            Json(RegisterDidResponse {
                did: row.did,
                created_at: row.created_at.to_rfc3339(),
            }),
        ));
    }

    // Already registered — only allow the exact same key.
    let existing = sqlx::query_as::<_, DidRow>(
        "SELECT did, public_key_b64, created_at FROM did_documents WHERE did = $1",
    )
    .bind(did.as_str())
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?
    .ok_or_else(|| ApiError::internal("DID disappeared during registration"))?;

    if existing.public_key_b64 != req.public_key_b64.trim() {
        return Err(ApiError::conflict(
            "DID is already registered with a different public key",
        ));
    }

    Ok((
        StatusCode::OK,
        Json(RegisterDidResponse {
            did: existing.did,
            created_at: existing.created_at.to_rfc3339(),
        }),
    ))
}

// ── DID lookup (public) ───────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DidDocumentResponse {
    pub did: String,
    pub public_key_b64: String,
    pub created_at: String,
}

pub async fn get_did_document(
    State(state): State<AppState>,
    Path(did): Path<String>,
) -> ApiResult<Json<DidDocumentResponse>> {
    let row = sqlx::query_as::<_, DidRow>(
        "SELECT did, public_key_b64, created_at FROM did_documents WHERE did = $1",
    )
    .bind(&did)
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?
    .ok_or_else(|| ApiError::not_found("DID not found"))?;

    Ok(Json(DidDocumentResponse {
        did: row.did,
        public_key_b64: row.public_key_b64,
        created_at: row.created_at.to_rfc3339(),
    }))
}

// ── Issue data grant ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct IssueGrantRequest {
    pub citizen_did: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub expires_in_seconds: u64,
}

#[derive(Serialize)]
pub struct GrantResponse {
    pub id: String,
    pub citizen_did: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub issued_at: String,
    pub expires_at: String,
    pub signature: String,
    pub issuer_did: String,
}

pub async fn issue_grant(
    State(state): State<AppState>,
    Json(req): Json<IssueGrantRequest>,
) -> ApiResult<(StatusCode, Json<GrantResponse>)> {
    let citizen_did = validate_did(&req.citizen_did)?;

    if !valid_dept_id(&req.requesting_dept_id) {
        return Err(ApiError::bad_request("invalid requesting_dept_id"));
    }
    if !valid_dept_id(&req.providing_dept_id) {
        return Err(ApiError::bad_request("invalid providing_dept_id"));
    }
    if req.requesting_dept_id == req.providing_dept_id {
        return Err(ApiError::bad_request(
            "requesting_dept_id and providing_dept_id must differ",
        ));
    }
    if req.scopes.is_empty() || req.scopes.len() > MAX_SCOPES {
        return Err(ApiError::bad_request(format!(
            "scopes must contain between 1 and {MAX_SCOPES} entries"
        )));
    }
    if let Some(bad) = req.scopes.iter().find(|s| !valid_scope(s)) {
        return Err(ApiError::bad_request(format!("invalid scope: {bad}")));
    }
    if !(MIN_GRANT_TTL..=MAX_GRANT_TTL).contains(&req.expires_in_seconds) {
        return Err(ApiError::bad_request(format!(
            "expires_in_seconds must be between {MIN_GRANT_TTL} and {MAX_GRANT_TTL}"
        )));
    }

    // De-duplicate scopes while preserving order.
    let mut scopes = req.scopes;
    scopes.sort();
    scopes.dedup();

    let dgc = state.issuer.issue_data_grant(
        citizen_did,
        &req.requesting_dept_id,
        &req.providing_dept_id,
        scopes,
        req.expires_in_seconds as i64,
    );

    let expires_dt = chrono::DateTime::from_timestamp(dgc.expires_at, 0)
        .ok_or_else(|| ApiError::internal("invalid expiry timestamp"))?;

    #[derive(sqlx::FromRow)]
    struct GrantRow {
        issued_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
    }

    let row = sqlx::query_as::<_, GrantRow>(
        r#"INSERT INTO data_grants (id, citizen_did, requesting_dept_id, providing_dept_id, scopes, expires_at, signature)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING issued_at, expires_at"#,
    )
    .bind(dgc.id)
    .bind(dgc.citizen_did.as_str())
    .bind(&dgc.requesting_dept_id)
    .bind(&dgc.providing_dept_id)
    .bind(&dgc.scopes)
    .bind(expires_dt)
    .bind(&dgc.signature)
    .fetch_one(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    Ok((
        StatusCode::CREATED,
        Json(GrantResponse {
            id: dgc.id.to_string(),
            citizen_did: dgc.citizen_did.to_string(),
            requesting_dept_id: dgc.requesting_dept_id,
            providing_dept_id: dgc.providing_dept_id,
            scopes: dgc.scopes,
            issued_at: row.issued_at.to_rfc3339(),
            expires_at: row.expires_at.to_rfc3339(),
            signature: dgc.signature,
            issuer_did: state.issuer.did.to_string(),
        }),
    ))
}

// ── Revoke data grant ─────────────────────────────────────────────────────────

pub async fn revoke_grant(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> ApiResult<StatusCode> {
    let result = sqlx::query(
        "UPDATE data_grants SET revoked_at = NOW() WHERE id = $1 AND revoked_at IS NULL",
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("grant not found or already revoked"));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ── Grant status (revocation check for department services) ───────────────────

#[derive(Deserialize)]
pub struct GrantStatusRequest {
    pub ids: Vec<uuid::Uuid>,
}

#[derive(Serialize)]
pub struct GrantStatusResponse {
    /// Grant ids that exist and have not been revoked.
    pub active: Vec<uuid::Uuid>,
    /// Grant ids that have been revoked.
    pub revoked: Vec<uuid::Uuid>,
    /// Grant ids unknown to this server (callers must treat these as invalid).
    pub unknown: Vec<uuid::Uuid>,
}

/// Batch revocation check used by department services before serving data.
pub async fn grant_status(
    State(state): State<AppState>,
    Json(req): Json<GrantStatusRequest>,
) -> ApiResult<Json<GrantStatusResponse>> {
    if req.ids.len() > 256 {
        return Err(ApiError::bad_request("at most 256 grant ids per request"));
    }

    #[derive(sqlx::FromRow)]
    struct StatusRow {
        id: uuid::Uuid,
        revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let rows =
        sqlx::query_as::<_, StatusRow>("SELECT id, revoked_at FROM data_grants WHERE id = ANY($1)")
            .bind(&req.ids)
            .fetch_all(&state.pool)
            .await
            .map_err(ApiError::internal)?;

    let mut active = Vec::new();
    let mut revoked = Vec::new();
    for row in rows {
        if row.revoked_at.is_some() {
            revoked.push(row.id);
        } else {
            active.push(row.id);
        }
    }

    let known: std::collections::HashSet<_> =
        active.iter().chain(revoked.iter()).copied().collect();
    let unknown = req
        .ids
        .into_iter()
        .filter(|id| !known.contains(id))
        .collect();

    Ok(Json(GrantStatusResponse {
        active,
        revoked,
        unknown,
    }))
}

// ── List grants for a citizen ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ListGrantsQuery {
    pub citizen_did: String,
}

pub async fn list_grants(
    State(state): State<AppState>,
    Query(q): Query<ListGrantsQuery>,
) -> ApiResult<Json<Vec<GrantResponse>>> {
    validate_did(&q.citizen_did)?;

    #[derive(sqlx::FromRow)]
    struct GrantRow {
        id: uuid::Uuid,
        citizen_did: String,
        requesting_dept_id: String,
        providing_dept_id: String,
        scopes: Vec<String>,
        issued_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
        signature: String,
    }

    let rows = sqlx::query_as::<_, GrantRow>(
        r#"SELECT id, citizen_did, requesting_dept_id, providing_dept_id, scopes,
                  issued_at, expires_at, signature
           FROM data_grants
           WHERE citizen_did = $1 AND revoked_at IS NULL
           ORDER BY issued_at DESC"#,
    )
    .bind(&q.citizen_did)
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    let issuer_did = state.issuer.did.to_string();
    let grants = rows
        .into_iter()
        .map(|r| GrantResponse {
            id: r.id.to_string(),
            citizen_did: r.citizen_did,
            requesting_dept_id: r.requesting_dept_id,
            providing_dept_id: r.providing_dept_id,
            scopes: r.scopes,
            issued_at: r.issued_at.to_rfc3339(),
            expires_at: r.expires_at.to_rfc3339(),
            signature: r.signature,
            issuer_did: issuer_did.clone(),
        })
        .collect();

    Ok(Json(grants))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dept_ids_are_strict() {
        assert!(valid_dept_id("ird"));
        assert!(valid_dept_id("statsnz"));
        assert!(valid_dept_id("te-arawhiti"));
        assert!(!valid_dept_id(""));
        assert!(!valid_dept_id("IRD"));
        assert!(!valid_dept_id("../etc"));
        assert!(!valid_dept_id("a b"));
    }

    #[test]
    fn scopes_are_validated() {
        assert!(valid_scope("ird:income"));
        assert!(valid_scope("ird:tax-summary"));
        assert!(!valid_scope("income"));
        assert!(!valid_scope(":income"));
        assert!(!valid_scope("ird:"));
        assert!(!valid_scope("ird:Income"));
    }

    #[test]
    fn public_keys_accept_standard_and_url_safe_base64() {
        use base64::Engine;
        use ed25519_dalek::SigningKey;

        let key = SigningKey::generate(&mut rand::rngs::OsRng);
        let verifying_key = key.verifying_key();
        let bytes = verifying_key.as_bytes();

        let standard = base64::engine::general_purpose::STANDARD.encode(bytes);
        let url_safe = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        assert_eq!(decode_public_key(&standard).unwrap().as_bytes(), bytes);
        assert_eq!(decode_public_key(&url_safe).unwrap().as_bytes(), bytes);
        assert!(decode_public_key("not base64 !!!").is_err());
        assert!(
            decode_public_key(&base64::engine::general_purpose::STANDARD.encode([0u8; 31]))
                .is_err()
        );
    }

    #[test]
    fn did_must_match_public_key() {
        use ed25519_dalek::SigningKey;

        let key = SigningKey::generate(&mut rand::rngs::OsRng);
        let did = gov_identity_core::GovDid::from_verifying_key(&key.verifying_key());
        assert_eq!(
            did.as_str(),
            gov_identity_core::GovDid::from_verifying_key(&key.verifying_key()).as_str()
        );
    }
}
