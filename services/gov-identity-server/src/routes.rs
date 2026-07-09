use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "gov-identity-server" })
}

// --- DID registration ---

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

pub async fn register_did(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterDidRequest>,
) -> Result<(StatusCode, Json<RegisterDidResponse>), StatusCode> {
    let row = sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
        r#"INSERT INTO did_documents (did, public_key_b64)
           VALUES ($1, $2)
           ON CONFLICT (did) DO UPDATE SET public_key_b64 = $2
           RETURNING created_at"#,
    )
    .bind(&req.did)
    .bind(&req.public_key_b64)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to register DID");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterDidResponse {
            did: req.did,
            created_at: row.to_rfc3339(),
        }),
    ))
}

// --- DID lookup ---

#[derive(Serialize)]
pub struct DidDocumentResponse {
    pub did: String,
    pub public_key_b64: String,
    pub created_at: String,
}

pub async fn get_did_document(
    State(pool): State<PgPool>,
    Path(did): Path<String>,
) -> Result<Json<DidDocumentResponse>, StatusCode> {
    #[derive(sqlx::FromRow)]
    struct DidRow {
        did: String,
        public_key_b64: String,
        created_at: chrono::DateTime<chrono::Utc>,
    }

    let row = sqlx::query_as::<_, DidRow>(
        "SELECT did, public_key_b64, created_at FROM did_documents WHERE did = $1",
    )
    .bind(&did)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to query DID");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(r) => Ok(Json(DidDocumentResponse {
            did: r.did,
            public_key_b64: r.public_key_b64,
            created_at: r.created_at.to_rfc3339(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// --- Issue data grant ---

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
}

pub async fn issue_grant(
    State(pool): State<PgPool>,
    Json(req): Json<IssueGrantRequest>,
) -> Result<(StatusCode, Json<GrantResponse>), StatusCode> {
    use gov_identity_core::{CredentialIssuer, GovDid};

    let citizen_did = GovDid::parse(&req.citizen_did).map_err(|_| StatusCode::BAD_REQUEST)?;

    // In production this would use the identity server's persistent keypair.
    let issuer = CredentialIssuer::generate();

    let dgc = issuer.issue_data_grant(
        citizen_did,
        &req.requesting_dept_id,
        &req.providing_dept_id,
        req.scopes.clone(),
        req.expires_in_seconds as i64,
    );

    let expires_dt = chrono::DateTime::from_timestamp(dgc.expires_at, 0)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

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
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to insert data grant");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
        }),
    ))
}

// --- Revoke data grant ---

pub async fn revoke_grant(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "UPDATE data_grants SET revoked_at = NOW() WHERE id = $1 AND revoked_at IS NULL",
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to revoke grant");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

// --- List grants for a citizen ---

#[derive(Deserialize)]
pub struct ListGrantsQuery {
    pub citizen_did: String,
}

pub async fn list_grants(
    State(pool): State<PgPool>,
    Query(q): Query<ListGrantsQuery>,
) -> Result<Json<Vec<GrantResponse>>, StatusCode> {
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
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "failed to list grants");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
        })
        .collect();

    Ok(Json(grants))
}
