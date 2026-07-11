use serde_json::Value;
use sha2::Digest;
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct SchemaRow {
    pub name: String,
    pub version: String,
    pub content: Value,
    pub required: Vec<String>,
    pub checksum: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// List the latest revision of every registered schema.
pub async fn list_latest(pool: &PgPool) -> Result<Vec<SchemaRow>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SchemaRow>(
        "SELECT name, version, content, required, checksum, created_at
         FROM schemas
         WHERE (name, revision) IN (
             SELECT name, MAX(revision) FROM schemas GROUP BY name
         )
         ORDER BY name",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_latest(pool: &PgPool, name: &str) -> Result<SchemaRow, sqlx::Error> {
    let row = sqlx::query_as::<_, SchemaRow>(
        "SELECT name, version, content, required, checksum, created_at
         FROM schemas WHERE name = $1
         ORDER BY revision DESC LIMIT 1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;
    Ok(row)
}

pub async fn get_version(
    pool: &PgPool,
    name: &str,
    version: &str,
) -> Result<SchemaRow, sqlx::Error> {
    let row = sqlx::query_as::<_, SchemaRow>(
        "SELECT name, version, content, required, checksum, created_at
         FROM schemas WHERE name = $1 AND version = $2
         ORDER BY revision DESC LIMIT 1",
    )
    .bind(name)
    .bind(version)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;
    Ok(row)
}

/// Upsert a schema version. `required` is derived from `content.required` when
/// present. The checksum is a SHA-256 of the canonical JSON content.
pub async fn register(
    pool: &PgPool,
    name: &str,
    version: &str,
    content: &Value,
) -> Result<SchemaRow, sqlx::Error> {
    let required: Vec<String> = content
        .get("required")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let canonical = serde_json::to_string(content).unwrap_or_default();
    let digest = sha2::Sha256::digest(canonical.as_bytes());
    let checksum = format!("{digest:x}");

    let row = sqlx::query_as::<_, SchemaRow>(
        "INSERT INTO schemas (name, version, content, required, checksum)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (name, version) DO UPDATE
           SET content = EXCLUDED.content,
               required = EXCLUDED.required,
               checksum = EXCLUDED.checksum
         RETURNING name, version, content, required, checksum, created_at",
    )
    .bind(name)
    .bind(version)
    .bind(content)
    .bind(&required)
    .bind(checksum)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
