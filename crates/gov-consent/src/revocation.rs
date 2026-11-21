//! Revocation checking against the identity server.
//!
//! Consent grants are issued by the identity server and can be revoked by the
//! citizen at any time. A department service must not serve data under a
//! revoked (or unknown) grant, so it asks the identity server which of the
//! presented grant ids are still active.
//!
//! The check **fails closed**: if the identity server cannot be reached, or
//! returns a grant id it does not know about, the request is denied.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_TIMEOUT_SECS: u64 = 5;
const DEFAULT_CACHE_SECS: u64 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Active,
    Revoked,
    Unknown,
}

#[derive(Serialize)]
struct StatusRequest<'a> {
    ids: &'a [Uuid],
}

#[derive(Deserialize)]
struct StatusResponse {
    #[serde(default)]
    active: Vec<Uuid>,
    #[serde(default)]
    revoked: Vec<Uuid>,
    #[serde(default)]
    unknown: Vec<Uuid>,
}

struct Cached {
    status: Status,
    checked_at: Instant,
}

/// Client for the identity server's batch grant-status endpoint.
pub struct RevocationClient {
    http: reqwest::Client,
    url: String,
    api_key: String,
    cache: Mutex<HashMap<Uuid, Cached>>,
    cache_ttl: Duration,
}

impl RevocationClient {
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        timeout: Duration,
        cache_ttl: Duration,
    ) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("building revocation HTTP client")?;
        Ok(Self {
            http,
            url: format!("{}/v1/grants/status", base_url.into().trim_end_matches('/')),
            api_key: api_key.into(),
            cache: Mutex::new(HashMap::new()),
            cache_ttl,
        })
    }

    /// Build from the environment.
    ///
    /// - `TPT__GOV__IDENTITY_SERVER_URL` — base URL (default `http://localhost:8081`)
    /// - `TPT__GOV__IDENTITY_API_KEY` — shared key (required)
    /// - `TPT__GOV__REVOCATION_TIMEOUT_SECS` — request timeout (default 5)
    /// - `TPT__GOV__REVOCATION_CACHE_SECS` — cache lifetime (default 30)
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("TPT__GOV__IDENTITY_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:8081".into());
        let api_key = std::env::var("TPT__GOV__IDENTITY_API_KEY")
            .context("TPT__GOV__IDENTITY_API_KEY is required to check grant revocation")?;
        let timeout = Duration::from_secs(
            std::env::var("TPT__GOV__REVOCATION_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_TIMEOUT_SECS),
        );
        let cache_ttl = Duration::from_secs(
            std::env::var("TPT__GOV__REVOCATION_CACHE_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_CACHE_SECS),
        );
        Self::new(base_url, api_key, timeout, cache_ttl)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<Uuid, Cached>> {
        self.cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn cached(&self, id: &Uuid, now: Instant) -> Option<Status> {
        let cache = self.lock();
        cache.get(id).and_then(|c| {
            if now.saturating_duration_since(c.checked_at) < self.cache_ttl {
                Some(c.status)
            } else {
                None
            }
        })
    }

    fn store(&self, statuses: &[(Uuid, Status)], now: Instant) {
        let mut cache = self.lock();
        for (id, status) in statuses {
            cache.insert(
                *id,
                Cached {
                    status: *status,
                    checked_at: now,
                },
            );
        }
    }

    /// Return the status of each id, fetching any not present in the cache.
    async fn statuses(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Status>> {
        let now = Instant::now();
        let mut result: HashMap<Uuid, Status> = HashMap::new();
        let mut missing: Vec<Uuid> = Vec::new();
        for id in ids {
            match self.cached(id, now) {
                Some(status) => {
                    result.insert(*id, status);
                }
                None => missing.push(*id),
            }
        }

        if missing.is_empty() {
            return Ok(result);
        }

        let response = self
            .http
            .post(&self.url)
            .header("x-identity-api-key", &self.api_key)
            .json(&StatusRequest { ids: &missing })
            .send()
            .await
            .context("grant-status request failed")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "identity server rejected grant-status request: {}",
                response.status()
            );
        }

        let body: StatusResponse = response
            .json()
            .await
            .context("invalid grant-status response")?;

        let mut fetched: Vec<(Uuid, Status)> = Vec::with_capacity(missing.len());
        for id in &body.active {
            fetched.push((*id, Status::Active));
        }
        for id in &body.revoked {
            fetched.push((*id, Status::Revoked));
        }
        for id in &body.unknown {
            fetched.push((*id, Status::Unknown));
        }
        // Any requested id the server did not classify is treated as unknown.
        for id in &missing {
            if !fetched.iter().any(|(known, _)| known == id) {
                fetched.push((*id, Status::Unknown));
            }
        }

        self.store(&fetched, now);
        for (id, status) in fetched {
            result.insert(id, status);
        }
        Ok(result)
    }

    /// Verify that every grant id is still active.
    ///
    /// Returns an error when any grant is revoked or unknown to the identity
    /// server, or when the identity server cannot be reached.
    pub async fn ensure_active(&self, ids: &[Uuid]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let statuses = self.statuses(ids).await?;
        let invalid: HashSet<Uuid> = statuses
            .iter()
            .filter(|(_, status)| **status != Status::Active)
            .map(|(id, _)| *id)
            .collect();
        if !invalid.is_empty() {
            anyhow::bail!("consent grant revoked or unknown: {invalid:?}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_marks_expiry() {
        let client = RevocationClient::new(
            "http://127.0.0.1:1",
            "test-key",
            Duration::from_millis(50),
            Duration::from_secs(60),
        )
        .unwrap();
        let id = Uuid::new_v4();
        let now = Instant::now();
        client.store(&[(id, Status::Active)], now);
        assert_eq!(client.cached(&id, now), Some(Status::Active));
        assert_eq!(client.cached(&id, now + Duration::from_secs(61)), None);
    }

    #[tokio::test]
    async fn empty_id_list_is_active() {
        let client = RevocationClient::new(
            "http://127.0.0.1:1",
            "test-key",
            Duration::from_millis(50),
            Duration::from_secs(60),
        )
        .unwrap();
        assert!(client.ensure_active(&[]).await.is_ok());
    }
}
