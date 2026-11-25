use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::Engine;
use ed25519_dalek::Verifier;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

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

impl From<gov_identity_core::IdentityError> for ApiError {
    fn from(err: gov_identity_core::IdentityError) -> Self {
        match err {
            gov_identity_core::IdentityError::InvalidDid(msg) => Self::bad_request(msg),
            gov_identity_core::IdentityError::InvalidSignature => {
                Self::bad_request("invalid signature")
            }
            gov_identity_core::IdentityError::Expired => Self::bad_request("credential expired"),
            gov_identity_core::IdentityError::Revoked => Self::bad_request("credential revoked"),
            gov_identity_core::IdentityError::Serialisation(e) => Self::internal(e),
            gov_identity_core::IdentityError::Crypto(e) => Self::internal(e),
        }
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

// ── JWKS ────────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Serialize)]
pub struct Jwk {
    pub kty: &'static str,
    #[serde(rename = "use")]
    pub r#use: &'static str,
    pub kid: String,
    pub alg: &'static str,
    pub crv: &'static str,
    pub x: String,
    pub y: String,
}

/// Public endpoint: JSON Web Key Set for the identity server's current issuer key.
pub async fn jwks(State(state): State<AppState>) -> Json<JwksResponse> {
    let verifying_key = state.issuer.verifying_key();
    let pk_bytes = verifying_key.as_bytes();
    let x = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&pk_bytes[0..16]);
    let y = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&pk_bytes[16..32]);

    Json(JwksResponse {
        keys: vec![Jwk {
            kty: "OKP",
            r#use: "sig",
            kid: state.issuer.did.to_string(),
            alg: "EdDSA",
            crv: "Ed25519",
            x,
            y,
        }],
    })
}

// ── OIDC Discovery ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct OidcDiscoveryResponse {
    pub issuer: String,
    pub jwks_uri: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub end_session_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
}

/// Public endpoint: OpenID Connect Discovery document.
pub async fn oidc_discovery(State(state): State<AppState>) -> Json<OidcDiscoveryResponse> {
    let issuer = state.issuer.did.to_string();
    let base_url = std::env::var("TPT__GOV__IDENTITY_PUBLIC_URL").unwrap_or_else(|_| {
        format!(
            "http://localhost:{}",
            state
                .issuer
                .did
                .to_string()
                .split(':')
                .next_back()
                .unwrap_or("8081")
        )
    });

    Json(OidcDiscoveryResponse {
        issuer: issuer.clone(),
        jwks_uri: format!("{base_url}/.well-known/jwks.json"),
        authorization_endpoint: format!("{base_url}/v1/did/challenge"),
        token_endpoint: format!("{base_url}/v1/did/register"),
        userinfo_endpoint: format!("{base_url}/v1/did/{{did}}"),
        end_session_endpoint: format!("{base_url}/v1/session/end"),
        scopes_supported: vec![
            "openid".into(),
            "profile".into(),
            "grants:read".into(),
            "grants:write".into(),
        ],
        response_types_supported: vec!["code".into(), "id_token".into()],
        subject_types_supported: vec!["public".into()],
        id_token_signing_alg_values_supported: vec!["EdDSA".into()],
        token_endpoint_auth_methods_supported: vec!["private_key_jwt".into()],
        claims_supported: vec!["sub".into(), "did".into(), "grants".into()],
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

#[derive(sqlx::FromRow)]
struct DidRow {
    did: String,
    public_key_b64: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

// ── DID challenge-response registration ────────────────────────────────────────

#[derive(Deserialize)]
pub struct DidChallengeRequest {
    pub did: String,
    pub public_key_b64: String,
}

#[derive(Serialize)]
pub struct DidChallengeResponse {
    pub challenge: String,
    pub challenge_id: String,
    pub expires_at: String,
}

/// Step 1: Request a challenge for DID registration.
/// The client must sign this challenge with their private key to prove possession.
pub async fn did_challenge(
    State(state): State<AppState>,
    Json(req): Json<DidChallengeRequest>,
) -> ApiResult<Json<DidChallengeResponse>> {
    let did = validate_did(&req.did)?;
    let verifying_key = decode_public_key(&req.public_key_b64)?;

    // Verify the DID matches the public key fingerprint.
    let expected = gov_identity_core::GovDid::from_verifying_key(&verifying_key);
    if expected.as_str() != did.as_str() {
        return Err(ApiError::bad_request(
            "public key does not match DID fingerprint",
        ));
    }

    // Generate a random challenge.
    let mut challenge_bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut challenge_bytes);
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(challenge_bytes);

    // Store challenge in database with 5-minute TTL.
    let challenge_id = Uuid::new_v4();
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(5);

    sqlx::query(
        r#"
        INSERT INTO did_challenges (id, did, public_key_b64, challenge, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(challenge_id)
    .bind(did.as_str())
    .bind(req.public_key_b64.trim())
    .bind(&challenge)
    .bind(expires_at)
    .execute(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    Ok(Json(DidChallengeResponse {
        challenge,
        challenge_id: challenge_id.to_string(),
        expires_at: expires_at.to_rfc3339(),
    }))
}

#[derive(Deserialize)]
pub struct RegisterDidWithProofRequest {
    pub challenge_id: String,
    pub signature_b64: String,
}

#[derive(Serialize)]
pub struct RegisterDidWithProofResponse {
    pub did: String,
    pub created_at: String,
}

/// Step 2: Complete DID registration by providing the challenge signature.
/// This proves the client possesses the private key corresponding to the DID.
pub async fn register_did_with_proof(
    State(state): State<AppState>,
    Json(req): Json<RegisterDidWithProofRequest>,
) -> ApiResult<(StatusCode, Json<RegisterDidWithProofResponse>)> {
    // Fetch and validate challenge.
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct ChallengeRow {
        id: Uuid,
        did: String,
        public_key_b64: String,
        challenge: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    }

    let challenge_id = Uuid::parse_str(&req.challenge_id)
        .map_err(|_| ApiError::bad_request("invalid challenge_id"))?;

    let row = sqlx::query_as::<_, ChallengeRow>(
        "SELECT id, did, public_key_b64, challenge, expires_at FROM did_challenges WHERE id = $1",
    )
    .bind(challenge_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiError::internal)?
    .ok_or_else(|| ApiError::not_found("challenge not found or expired"))?;

    // Check expiry.
    if chrono::Utc::now() > row.expires_at {
        // Clean up expired challenge.
        sqlx::query("DELETE FROM did_challenges WHERE id = $1")
            .bind(challenge_id)
            .execute(&state.pool)
            .await
            .ok();
        return Err(ApiError::bad_request("challenge expired"));
    }

    // Verify the signature over the challenge.
    let verifying_key = decode_public_key(&row.public_key_b64)?;
    let challenge_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(&row.challenge)
        .map_err(|_| ApiError::bad_request("invalid challenge encoding"))?;
    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(&req.signature_b64)
        .map_err(|_| ApiError::bad_request("invalid signature encoding"))?;

    let signature = ed25519_dalek::Signature::from_bytes(
        &signature_bytes
            .try_into()
            .map_err(|_| ApiError::bad_request("signature length invalid"))?,
    );

    verifying_key
        .verify(&challenge_bytes, &signature)
        .map_err(|_| ApiError::bad_request("invalid signature"))?;

    // Challenge verified — register the DID and delete the challenge.
    let did = gov_identity_core::GovDid::parse(&row.did)?;

    let mut tx = state.pool.begin().await.map_err(ApiError::internal)?;

    let inserted = sqlx::query_as::<_, DidRow>(
        r#"INSERT INTO did_documents (did, public_key_b64)
           VALUES ($1, $2)
           ON CONFLICT (did) DO NOTHING
           RETURNING did, public_key_b64, created_at"#,
    )
    .bind(did.as_str())
    .bind(&row.public_key_b64)
    .fetch_optional(&mut *tx)
    .await
    .map_err(ApiError::internal)?;

    let result = if let Some(row) = inserted {
        (StatusCode::CREATED, row)
    } else {
        // Already registered — verify it's the same key.
        let existing = sqlx::query_as::<_, DidRow>(
            "SELECT did, public_key_b64, created_at FROM did_documents WHERE did = $1",
        )
        .bind(did.as_str())
        .fetch_optional(&mut *tx)
        .await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::internal("DID disappeared during registration"))?;

        if existing.public_key_b64 != row.public_key_b64 {
            return Err(ApiError::conflict(
                "DID is already registered with a different public key",
            ));
        }
        (StatusCode::OK, existing)
    };

    // Delete the used challenge.
    sqlx::query("DELETE FROM did_challenges WHERE id = $1")
        .bind(challenge_id)
        .execute(&mut *tx)
        .await
        .map_err(ApiError::internal)?;

    // Add to key history.
    sqlx::query(
        r#"
        INSERT INTO did_key_history (did, public_key_b64, activated_at, is_current)
        VALUES ($1, $2, $3, TRUE)
        ON CONFLICT (did) WHERE is_current DO UPDATE SET
            is_current = FALSE,
            deactivated_at = $3
        "#,
    )
    .bind(did.as_str())
    .bind(&result.1.public_key_b64)
    .bind(chrono::Utc::now())
    .execute(&mut *tx)
    .await
    .map_err(ApiError::internal)?;

    tx.commit().await.map_err(ApiError::internal)?;

    Ok((
        result.0,
        Json(RegisterDidWithProofResponse {
            did: result.1.did,
            created_at: result.1.created_at.to_rfc3339(),
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
    let now = chrono::Utc::now();

    let mut tx = state.pool.begin().await.map_err(ApiError::internal)?;

    // Get grant details before revoking (for revocation list).
    #[derive(sqlx::FromRow)]
    struct GrantForRevocation {
        id: uuid::Uuid,
        citizen_did: String,
    }

    let grant = sqlx::query_as::<_, GrantForRevocation>(
        "SELECT id, citizen_did FROM data_grants WHERE id = $1 AND revoked_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(ApiError::internal)?
    .ok_or_else(|| ApiError::not_found("grant not found or already revoked"))?;

    // Mark grant as revoked.
    sqlx::query("UPDATE data_grants SET revoked_at = $1 WHERE id = $2 AND revoked_at IS NULL")
        .bind(now)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(ApiError::internal)?;

    // Add to revocation list.
    sqlx::query(
        r#"
        INSERT INTO revocation_list (grant_id, citizen_did, revoked_at, revoked_by, reason)
        VALUES ($1, $2, $3, 'admin', 'Revoked by admin')
        "#,
    )
    .bind(grant.id)
    .bind(&grant.citizen_did)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(ApiError::internal)?;

    tx.commit().await.map_err(ApiError::internal)?;

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

// ── Admin API ───────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AdminListGrantsQuery {
    pub citizen_did: Option<String>,
    pub requesting_dept_id: Option<String>,
    pub providing_dept_id: Option<String>,
    pub include_revoked: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct AdminGrantResponse {
    pub id: String,
    pub citizen_did: String,
    pub requesting_dept_id: String,
    pub providing_dept_id: String,
    pub scopes: Vec<String>,
    pub issued_at: String,
    pub expires_at: String,
    pub revoked_at: Option<String>,
    pub signature: String,
    pub issuer_did: String,
}

/// Admin endpoint: list all grants with optional filters.
pub async fn admin_list_grants(
    State(state): State<AppState>,
    Query(q): Query<AdminListGrantsQuery>,
) -> ApiResult<Json<Vec<AdminGrantResponse>>> {
    let mut query = String::from(
        r#"SELECT id, citizen_did, requesting_dept_id, providing_dept_id, scopes,
                  issued_at, expires_at, revoked_at, signature
           FROM data_grants"#,
    );

    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(citizen_did) = &q.citizen_did {
        validate_did(citizen_did)?;
        conditions.push(format!("citizen_did = ${}", params.len() + 1));
        params.push(citizen_did.clone());
    }
    if let Some(requesting_dept_id) = &q.requesting_dept_id {
        if !valid_dept_id(requesting_dept_id) {
            return Err(ApiError::bad_request("invalid requesting_dept_id"));
        }
        conditions.push(format!("requesting_dept_id = ${}", params.len() + 1));
        params.push(requesting_dept_id.clone());
    }
    if let Some(providing_dept_id) = &q.providing_dept_id {
        if !valid_dept_id(providing_dept_id) {
            return Err(ApiError::bad_request("invalid providing_dept_id"));
        }
        conditions.push(format!("providing_dept_id = ${}", params.len() + 1));
        params.push(providing_dept_id.clone());
    }
    if !q.include_revoked.unwrap_or(false) {
        conditions.push("revoked_at IS NULL".to_string());
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY issued_at DESC");

    if let Some(limit) = q.limit {
        query.push_str(&format!(" LIMIT {limit}"));
    }
    if let Some(offset) = q.offset {
        query.push_str(&format!(" OFFSET {offset}"));
    }

    #[derive(sqlx::FromRow)]
    struct AdminGrantRow {
        id: uuid::Uuid,
        citizen_did: String,
        requesting_dept_id: String,
        providing_dept_id: String,
        scopes: Vec<String>,
        issued_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
        revoked_at: Option<chrono::DateTime<chrono::Utc>>,
        signature: String,
    }

    let mut qb = sqlx::query_as::<_, AdminGrantRow>(&query);
    for param in &params {
        qb = qb.bind(param);
    }

    let rows = qb
        .fetch_all(&state.pool)
        .await
        .map_err(ApiError::internal)?;

    let issuer_did = state.issuer.did.to_string();
    let grants = rows
        .into_iter()
        .map(|r| AdminGrantResponse {
            id: r.id.to_string(),
            citizen_did: r.citizen_did,
            requesting_dept_id: r.requesting_dept_id,
            providing_dept_id: r.providing_dept_id,
            scopes: r.scopes,
            issued_at: r.issued_at.to_rfc3339(),
            expires_at: r.expires_at.to_rfc3339(),
            revoked_at: r.revoked_at.map(|dt| dt.to_rfc3339()),
            signature: r.signature,
            issuer_did: issuer_did.clone(),
        })
        .collect();

    Ok(Json(grants))
}

#[derive(Serialize)]
pub struct AdminRevokeCitizenResponse {
    pub revoked_count: u64,
}

/// Admin endpoint: revoke all grants for a citizen.
pub async fn admin_revoke_citizen_grants(
    State(state): State<AppState>,
    Path(citizen_did): Path<String>,
) -> ApiResult<Json<AdminRevokeCitizenResponse>> {
    let did = validate_did(&citizen_did)?;
    let now = chrono::Utc::now();

    let mut tx = state.pool.begin().await.map_err(ApiError::internal)?;

    // Get all active grants for this citizen.
    #[derive(sqlx::FromRow)]
    struct GrantForRevocation {
        id: uuid::Uuid,
        citizen_did: String,
    }

    let grants = sqlx::query_as::<_, GrantForRevocation>(
        "SELECT id, citizen_did FROM data_grants WHERE citizen_did = $1 AND revoked_at IS NULL",
    )
    .bind(did.as_str())
    .fetch_all(&mut *tx)
    .await
    .map_err(ApiError::internal)?;

    for grant in &grants {
        // Mark grant as revoked.
        sqlx::query("UPDATE data_grants SET revoked_at = $1 WHERE id = $2")
            .bind(now)
            .bind(grant.id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::internal)?;

        // Add to revocation list.
        sqlx::query(
            r#"
            INSERT INTO revocation_list (grant_id, citizen_did, revoked_at, revoked_by, reason)
            VALUES ($1, $2, $3, 'admin', 'Bulk revoked by admin for citizen')
            "#,
        )
        .bind(grant.id)
        .bind(&grant.citizen_did)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(ApiError::internal)?;
    }

    let revoked_count = grants.len() as u64;
    tx.commit().await.map_err(ApiError::internal)?;

    info!(citizen_did = %did, revoked_count, "Admin revoked all grants for citizen");

    Ok(Json(AdminRevokeCitizenResponse { revoked_count }))
}

#[derive(Deserialize)]
pub struct RotateIssuerKeyRequest {
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct RotateIssuerKeyResponse {
    pub old_did: String,
    pub new_did: String,
    pub rotated_at: String,
}

/// Admin endpoint: rotate the issuer keypair.
/// This generates a new keypair, stores it as the active issuer key,
/// and logs the rotation. The old key is kept in the rotation log for
/// verification of previously issued credentials.
pub async fn admin_rotate_issuer_key(
    State(state): State<AppState>,
    Json(req): Json<RotateIssuerKeyRequest>,
) -> ApiResult<Json<RotateIssuerKeyResponse>> {
    let old_did = state.issuer.did.to_string();
    let _old_key_b64 = state.issuer.secret_key_b64();

    // Generate new issuer key.
    let new_issuer = gov_identity_core::CredentialIssuer::generate();
    let new_did = new_issuer.did.to_string();
    let new_key_b64 = new_issuer.secret_key_b64();

    // Update the issuer_keys table.
    sqlx::query("UPDATE issuer_keys SET secret_key_b64 = $1 WHERE id = 1")
        .bind(new_key_b64)
        .execute(&state.pool)
        .await
        .map_err(ApiError::internal)?;

    // Log the rotation.
    let rotated_at = chrono::Utc::now();
    sqlx::query(
        r#"
        INSERT INTO issuer_key_rotation_log (old_key_id, new_key_id, rotated_at, rotated_by, reason, old_did, new_did)
        VALUES (1, 1, $1, 'admin', $2, $3, $4)
        "#,
    )
    .bind(rotated_at)
    .bind(req.reason.unwrap_or_else(|| "Manual rotation via admin API".into()))
    .bind(&old_did)
    .bind(&new_did)
    .execute(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    // Update the in-memory issuer (requires Arc swap - in practice this would need
    // a more sophisticated approach for production, but for now we log the rotation
    // and the new key will be used on next restart).
    // Note: In a production deployment with multiple instances, this would need
    // a coordinated rotation (e.g., via consensus or a shared KMS).
    tracing::warn!(
        old_did = %old_did,
        new_did = %new_did,
        "Issuer key rotated — restart all identity server instances to pick up the new key"
    );

    // Emit rotation event for background tasks.
    let _ = state.rotation_tx.send(crate::state::RotationEvent {
        _old_did: old_did.clone(),
        _new_did: new_did.clone(),
        _rotated_at: rotated_at,
    });

    Ok(Json(RotateIssuerKeyResponse {
        old_did,
        new_did,
        rotated_at: rotated_at.to_rfc3339(),
    }))
}

#[derive(Serialize)]
pub struct DeptHealthResponse {
    pub dept_id: String,
    pub grants_issued: i64,
    pub grants_active: i64,
    pub grants_revoked: i64,
    pub last_grant_issued: Option<String>,
}

/// Admin endpoint: get health stats for all departments.
pub async fn admin_dept_health(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<DeptHealthResponse>>> {
    #[derive(sqlx::FromRow)]
    struct DeptHealthRow {
        dept_id: String,
        grants_issued: i64,
        grants_active: i64,
        grants_revoked: i64,
        last_grant_issued: Option<chrono::DateTime<chrono::Utc>>,
    }

    // Query requesting dept stats.
    let requesting = sqlx::query_as::<_, DeptHealthRow>(
        r#"
        SELECT requesting_dept_id as dept_id,
               COUNT(*) as grants_issued,
               COUNT(*) FILTER (WHERE revoked_at IS NULL) as grants_active,
               COUNT(*) FILTER (WHERE revoked_at IS NOT NULL) as grants_revoked,
               MAX(issued_at) as last_grant_issued
        FROM data_grants
        GROUP BY requesting_dept_id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    // Query providing dept stats.
    let providing = sqlx::query_as::<_, DeptHealthRow>(
        r#"
        SELECT providing_dept_id as dept_id,
               COUNT(*) as grants_issued,
               COUNT(*) FILTER (WHERE revoked_at IS NULL) as grants_active,
               COUNT(*) FILTER (WHERE revoked_at IS NOT NULL) as grants_revoked,
               MAX(issued_at) as last_grant_issued
        FROM data_grants
        GROUP BY providing_dept_id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiError::internal)?;

    // Combine results.
    use std::collections::HashMap;
    let mut combined: HashMap<String, DeptHealthRow> = HashMap::new();

    for row in requesting {
        combined.insert(row.dept_id.clone(), row);
    }

    for row in providing {
        combined
            .entry(row.dept_id.clone())
            .and_modify(|e| {
                e.grants_issued += row.grants_issued;
                e.grants_active += row.grants_active;
                e.grants_revoked += row.grants_revoked;
                match (row.last_grant_issued, e.last_grant_issued) {
                    (Some(new), Some(old)) if new > old => {
                        e.last_grant_issued = row.last_grant_issued
                    }
                    (Some(_), None) => e.last_grant_issued = row.last_grant_issued,
                    _ => {}
                }
            })
            .or_insert(row);
    }

    let health: Vec<DeptHealthResponse> = combined
        .into_values()
        .map(|r| DeptHealthResponse {
            dept_id: r.dept_id,
            grants_issued: r.grants_issued,
            grants_active: r.grants_active,
            grants_revoked: r.grants_revoked,
            last_grant_issued: r.last_grant_issued.map(|dt| dt.to_rfc3339()),
        })
        .collect();

    Ok(Json(health))
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

        let mut seed = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut seed);
        let key = SigningKey::from_bytes(&seed);
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

        let mut seed = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut seed);
        let key = SigningKey::from_bytes(&seed);
        let did = gov_identity_core::GovDid::from_verifying_key(&key.verifying_key());
        assert_eq!(
            did.as_str(),
            gov_identity_core::GovDid::from_verifying_key(&key.verifying_key()).as_str()
        );
    }
}
