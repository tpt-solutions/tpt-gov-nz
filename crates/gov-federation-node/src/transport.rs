//! HTTP mock transport for Phase 1 local development.
//!
//! Provides a signature-verified request/response channel between federation
//! nodes over plain HTTP. This lets departments exchange [`FederationEnvelope`]
//! messages on a local machine without the QUIC transport (Phase 2).
//!
//! Wire contract:
//! - A node accepts inbound envelopes at [`ENVELOPE_PATH`] via `POST`.
//! - The body is a JSON-encoded [`FederationEnvelope`].
//! - The receiver verifies the sender's Ed25519 signature against the peer's
//!   registered public key, dispatches to the node's handler, signs the
//!   response envelope, and returns it as JSON.

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use ed25519_dalek::VerifyingKey;
use gov_consent::RevocationClient;
use gov_federation_core::{FederationEnvelope, FederationError, NodeKeypair};
use tracing::{debug, warn};

use crate::config::PeerConfig;

/// Maximum allowed clock skew for envelope timestamps (seconds).
pub const MAX_TIMESTAMP_SKEW_SECS: i64 = 60;

/// Size of the replay window (how many recent message IDs to track).
pub const REPLAY_WINDOW_SIZE: usize = 10_000;

/// Default rate limit for federation transport (requests per minute per peer).
pub const DEFAULT_FEDERATION_RATE_LIMIT_RPM: u32 = 60;

/// Maximum number of peers tracked by the federation rate limiter.
pub const DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS: usize = 100;

/// Replay protection state for deduplicating message IDs.
pub struct ReplayProtection {
    seen: HashSet<uuid::Uuid>,
    max_size: usize,
}

impl ReplayProtection {
    pub fn new(max_size: usize) -> Self {
        Self {
            seen: HashSet::with_capacity(max_size),
            max_size,
        }
    }

    /// Check if a message ID has been seen. If not, add it and return Ok(()).
    /// If already seen, return an error.
    pub fn check_and_insert(&mut self, msg_id: uuid::Uuid) -> Result<(), String> {
        if self.seen.contains(&msg_id) {
            return Err("replay detected: duplicate message_id".into());
        }
        if self.seen.len() >= self.max_size {
            let to_remove: Vec<_> = self.seen.iter().take(self.max_size / 2).cloned().collect();
            for id in to_remove {
                self.seen.remove(&id);
            }
        }
        self.seen.insert(msg_id);
        Ok(())
    }
}

/// Federation rate limiter for per-peer rate limiting.
pub struct FederationRateLimiter {
    windows: Arc<Mutex<HashMap<String, Window>>>,
    max_requests: u32,
    window: Duration,
    max_peers: usize,
}

pub struct Window {
    count: u32,
    reset_at: Instant,
}

impl FederationRateLimiter {
    pub fn new(max_requests: u32, window: Duration, max_peers: usize) -> Self {
        Self {
            windows: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
            max_peers: max_peers.max(1),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, Window>> {
        self.windows
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn make_room(windows: &mut HashMap<String, Window>, now: Instant, max_peers: usize) {
        if windows.len() < max_peers {
            return;
        }
        windows.retain(|_, w| now < w.reset_at);
        if windows.len() < max_peers {
            return;
        }
        if let Some(key) = windows
            .iter()
            .min_by_key(|(_, w)| w.reset_at)
            .map(|(k, _)| k.clone())
        {
            windows.remove(&key);
        }
    }

    /// Check if a request from `peer_id` is allowed.
    /// Returns `Ok(remaining)` when allowed, or `Err(retry_after_secs)` when rate limited.
    pub fn check(&self, peer_id: &str) -> Result<u32, u64> {
        let now = Instant::now();
        let mut windows = self.lock();
        Self::make_room(&mut windows, now, self.max_peers);

        let w = windows
            .entry(peer_id.to_string())
            .or_insert_with(|| Window {
                count: 0,
                reset_at: now + self.window,
            });

        if now >= w.reset_at {
            w.count = 0;
            w.reset_at = now + self.window;
        }

        if w.count >= self.max_requests {
            let retry = w.reset_at.saturating_duration_since(now).as_secs() + 1;
            Err(retry)
        } else {
            w.count += 1;
            Ok(self.max_requests - w.count)
        }
    }
}

/// Path where a node accepts inbound federation envelopes over the mock transport.
pub const ENVELOPE_PATH: &str = "/federation/v1/envelope";

/// Async handler invoked for each inbound, signature-verified envelope.
///
/// Returns the response envelope to send back to the caller. The transport
/// signs the returned envelope before it leaves the node, so handlers should
/// leave the `signature` field empty.
pub type EnvelopeHandler = Arc<
    dyn Fn(
            FederationEnvelope,
        )
            -> Pin<Box<dyn Future<Output = Result<FederationEnvelope, FederationError>> + Send>>
        + Send
        + Sync,
>;

/// Decode a base64 Ed25519 public key into a [`VerifyingKey`].
pub fn verifying_key_from_b64(b64: &str) -> Result<VerifyingKey, FederationError> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| FederationError::Crypto(format!("invalid base64 public key: {e}")))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| FederationError::Crypto("public key length invalid".into()))?;
    VerifyingKey::from_bytes(&arr)
        .map_err(|e| FederationError::Crypto(format!("invalid public key: {e}")))
}

/// Shared state for the inbound HTTP transport server.
#[derive(Clone)]
pub struct InboundState {
    pub keypair: Arc<NodeKeypair>,
    /// Peers indexed by `dept_id`, used to verify inbound sender signatures.
    pub peers: Arc<HashMap<String, PeerConfig>>,
    pub handler: EnvelopeHandler,
    /// Replay protection for message ID deduplication.
    pub replay_protection: Arc<tokio::sync::Mutex<ReplayProtection>>,
    /// Per-peer rate limiter for federation transport.
    pub federation_rate_limiter: Arc<FederationRateLimiter>,
    /// Revocation client for consent grant ID verification (optional for testing).
    pub revocation_client: Option<Arc<RevocationClient>>,
}

/// Build the axum router exposing the inbound envelope endpoint.
///
/// The replay window, rate limiter and revocation client are taken from `state`
/// as given, so the caller controls their lifetime (one shared instance per node).
/// If no revocation client is supplied, one is built from the environment.
pub fn router(state: InboundState) -> Router {
    let state = InboundState {
        revocation_client: state
            .revocation_client
            .or_else(|| RevocationClient::from_env().ok().map(Arc::new)),
        ..state
    };
    Router::new()
        .route(ENVELOPE_PATH, post(receive_envelope))
        .with_state(state)
}

/// Policy checks applied to every inbound envelope after its signature has been
/// verified. Shared by the HTTP mock transport and the QUIC transport.
///
/// Fails closed. Checks, in order:
/// 1. the envelope is addressed to this node (`to_dept_id`);
/// 2. the timestamp is within [`MAX_TIMESTAMP_SKEW_SECS`] of now;
/// 3. the sender is within its per-peer rate limit;
/// 4. the message ID has not been seen before (replay);
/// 5. every consent grant the envelope cites is still active.
///
/// Signature verification must happen first, so that forged envelopes cannot
/// consume replay-window or rate-limit slots.
pub async fn check_inbound_policy(
    state: &InboundState,
    envelope: &FederationEnvelope,
) -> Result<(), (StatusCode, String)> {
    if envelope.to_dept_id != state.keypair.dept_id {
        warn!(
            from = %envelope.from_dept_id,
            to = %envelope.to_dept_id,
            "rejecting envelope not addressed to this node"
        );
        return Err((
            StatusCode::MISDIRECTED_REQUEST,
            "envelope is not addressed to this node".into(),
        ));
    }

    let now_ms = chrono::Utc::now().timestamp_millis();
    if (now_ms - envelope.timestamp_ms).abs() > MAX_TIMESTAMP_SKEW_SECS * 1000 {
        warn!(from = %envelope.from_dept_id, "rejecting envelope outside timestamp skew");
        return Err((
            StatusCode::BAD_REQUEST,
            "envelope timestamp is outside the allowed skew".into(),
        ));
    }

    if let Err(retry_after) = state.federation_rate_limiter.check(&envelope.from_dept_id) {
        warn!(from = %envelope.from_dept_id, "federation rate limit exceeded");
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            format!("rate limit exceeded; retry in {retry_after}s"),
        ));
    }

    if state
        .replay_protection
        .lock()
        .await
        .check_and_insert(envelope.message_id)
        .is_err()
    {
        warn!(from = %envelope.from_dept_id, msg_id = %envelope.message_id, "replay detected");
        return Err((
            StatusCode::CONFLICT,
            "replay detected: duplicate message_id".into(),
        ));
    }

    if let (Some(revocation), false) = (
        &state.revocation_client,
        envelope.consent_grant_ids.is_empty(),
    ) && revocation
        .ensure_active(&envelope.consent_grant_ids)
        .await
        .is_err()
    {
        warn!(from = %envelope.from_dept_id, "consent grant revoked or unknown");
        return Err((
            StatusCode::FORBIDDEN,
            "consent grant is revoked or unknown".into(),
        ));
    }

    Ok(())
}

async fn receive_envelope(
    State(state): State<InboundState>,
    Json(envelope): Json<FederationEnvelope>,
) -> Result<Json<FederationEnvelope>, (StatusCode, String)> {
    // The sender must be a known peer so we can verify its signature.
    let peer = state.peers.get(&envelope.from_dept_id).ok_or_else(|| {
        warn!(from = %envelope.from_dept_id, "rejecting envelope from unknown peer");
        (
            StatusCode::FORBIDDEN,
            format!("unknown peer: {}", envelope.from_dept_id),
        )
    })?;

    let sender_key = verifying_key_from_b64(&peer.public_key_b64)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    envelope.verify(&sender_key).map_err(|e| {
        warn!(from = %envelope.from_dept_id, error = %e, "envelope signature verification failed");
        (StatusCode::UNAUTHORIZED, e.to_string())
    })?;

    check_inbound_policy(&state, &envelope).await?;

    debug!(
        from = %envelope.from_dept_id,
        to = %envelope.to_dept_id,
        message_id = %envelope.message_id,
        "inbound envelope verified"
    );

    // Dispatch to the node handler, then sign the response on the way out.
    let mut response = (state.handler)(envelope)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    response.sign(&state.keypair.signing_key);

    Ok(Json(response))
}

/// HTTP client side of the mock transport.
#[derive(Clone)]
pub struct HttpMockTransport {
    client: reqwest::Client,
}

impl Default for HttpMockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpMockTransport {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Send a (already-signed) envelope to `peer` and await its signed response.
    ///
    /// The response signature is verified against the peer's public key before
    /// it is returned to the caller.
    pub async fn send(
        &self,
        peer: &PeerConfig,
        envelope: FederationEnvelope,
    ) -> Result<FederationEnvelope, FederationError> {
        let url = format!("{}{}", peer.http_endpoint(), ENVELOPE_PATH);

        let resp = self
            .client
            .post(&url)
            .json(&envelope)
            .send()
            .await
            .map_err(|e| FederationError::Transport(format!("send to {url} failed: {e}")))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(FederationError::Transport(format!(
                "peer {} returned {status}: {body}",
                peer.dept_id
            )));
        }

        let response: FederationEnvelope = resp
            .json()
            .await
            .map_err(|e| FederationError::Transport(format!("invalid response body: {e}")))?;

        let peer_key = verifying_key_from_b64(&peer.public_key_b64)?;
        response.verify(&peer_key)?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_federation_core::FederationMessageType;
    use uuid::Uuid;

    fn echo_handler() -> EnvelopeHandler {
        Arc::new(|env: FederationEnvelope| {
            Box::pin(async move {
                Ok(FederationEnvelope {
                    message_id: Uuid::new_v4(),
                    correlation_id: Some(env.message_id),
                    message_type: FederationMessageType::DataResponse,
                    from_dept_id: env.to_dept_id.clone(),
                    to_dept_id: env.from_dept_id.clone(),
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                    consent_grant_ids: env.consent_grant_ids.clone(),
                    payload_encrypted: String::new(),
                    signature: String::new(),
                })
            })
        })
    }

    async fn spawn_server(state: InboundState) -> std::net::SocketAddr {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = router(state);
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        // Give the accept loop a moment to be ready.
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        addr
    }

    #[tokio::test]
    async fn http_mock_round_trip() {
        // Server = ird, client = winz. Each side knows the other's public key.
        let server_kp = Arc::new(NodeKeypair::generate("ird"));
        let client_kp = NodeKeypair::generate("winz");

        let mut server_peers = HashMap::new();
        server_peers.insert(
            "winz".to_string(),
            PeerConfig {
                dept_id: "winz".into(),
                addr: "127.0.0.1:1".parse().unwrap(),
                public_key_b64: client_kp.public_key_b64(),
            },
        );

        let state = InboundState {
            keypair: server_kp.clone(),
            peers: Arc::new(server_peers),
            handler: echo_handler(),
            replay_protection: Arc::new(tokio::sync::Mutex::new(ReplayProtection::new(
                REPLAY_WINDOW_SIZE,
            ))),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: Some(Arc::new(
                RevocationClient::new(
                    "http://127.0.0.1:1",
                    "test-key",
                    Duration::from_secs(5),
                    Duration::from_secs(60),
                )
                .unwrap(),
            )),
        };
        let addr = spawn_server(state).await;

        let peer = PeerConfig {
            dept_id: "ird".into(),
            addr,
            public_key_b64: server_kp.public_key_b64(),
        };

        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let transport = HttpMockTransport::new();
        let resp = transport.send(&peer, env).await.unwrap();

        assert_eq!(resp.message_type, FederationMessageType::DataResponse);
        assert_eq!(resp.from_dept_id, "ird");
        assert_eq!(resp.to_dept_id, "winz");
        assert!(!resp.signature.is_empty());
    }

    #[tokio::test]
    async fn rejects_unknown_peer() {
        // Server knows no peers, so any inbound sender is rejected.
        let server_kp = Arc::new(NodeKeypair::generate("ird"));
        let client_kp = NodeKeypair::generate("winz");

        let state = InboundState {
            keypair: server_kp.clone(),
            peers: Arc::new(HashMap::new()),
            handler: echo_handler(),
            replay_protection: Arc::new(tokio::sync::Mutex::new(ReplayProtection::new(
                REPLAY_WINDOW_SIZE,
            ))),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: Some(Arc::new(
                RevocationClient::new(
                    "http://127.0.0.1:1",
                    "test-key",
                    Duration::from_secs(5),
                    Duration::from_secs(60),
                )
                .unwrap(),
            )),
        };
        let addr = spawn_server(state).await;

        let peer = PeerConfig {
            dept_id: "ird".into(),
            addr,
            public_key_b64: server_kp.public_key_b64(),
        };

        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let transport = HttpMockTransport::new();
        let err = transport.send(&peer, env).await.unwrap_err();
        assert!(matches!(err, FederationError::Transport(_)));
    }

    /// Inbound state for an `ird` node that knows `winz` as its only peer.
    /// Revocation is disabled so the tests stay offline.
    fn ird_state_for_winz(client_kp: &NodeKeypair) -> InboundState {
        let server_kp = Arc::new(NodeKeypair::generate("ird"));
        let mut peers = HashMap::new();
        peers.insert(
            "winz".to_string(),
            PeerConfig {
                dept_id: "winz".into(),
                addr: "127.0.0.1:1".parse().unwrap(),
                public_key_b64: client_kp.public_key_b64(),
            },
        );
        InboundState {
            keypair: server_kp,
            peers: Arc::new(peers),
            handler: echo_handler(),
            replay_protection: Arc::new(tokio::sync::Mutex::new(ReplayProtection::new(
                REPLAY_WINDOW_SIZE,
            ))),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: None,
        }
    }

    /// The `ird` server as seen from the `winz` client.
    fn ird_peer(addr: std::net::SocketAddr, server: &NodeKeypair) -> PeerConfig {
        PeerConfig {
            dept_id: "ird".into(),
            addr,
            public_key_b64: server.public_key_b64(),
        }
    }

    #[tokio::test]
    async fn rejects_envelope_not_addressed_to_this_node() {
        let client_kp = NodeKeypair::generate("winz");
        let state = ird_state_for_winz(&client_kp);
        let server_kp = state.keypair.clone();
        let addr = spawn_server(state).await;
        let peer = ird_peer(addr, &server_kp);

        let mut env =
            FederationEnvelope::new_request("winz", "dia", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let err = HttpMockTransport::new().send(&peer, env).await.unwrap_err();
        assert!(matches!(err, FederationError::Transport(_)));
    }

    #[tokio::test]
    async fn rejects_replayed_envelope() {
        let client_kp = NodeKeypair::generate("winz");
        let state = ird_state_for_winz(&client_kp);
        let server_kp = state.keypair.clone();
        let addr = spawn_server(state).await;
        let peer = ird_peer(addr, &server_kp);

        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let transport = HttpMockTransport::new();
        transport.send(&peer, env.clone()).await.unwrap();
        let err = transport.send(&peer, env).await.unwrap_err();
        assert!(matches!(err, FederationError::Transport(_)));
    }

    #[tokio::test]
    async fn rejects_stale_timestamp() {
        let client_kp = NodeKeypair::generate("winz");
        let state = ird_state_for_winz(&client_kp);
        let server_kp = state.keypair.clone();
        let addr = spawn_server(state).await;
        let peer = ird_peer(addr, &server_kp);

        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.timestamp_ms -= 10 * 60 * 1000;
        env.sign(&client_kp.signing_key);

        let err = HttpMockTransport::new().send(&peer, env).await.unwrap_err();
        assert!(matches!(err, FederationError::Transport(_)));
    }

    #[tokio::test]
    async fn rejects_bad_signature() {
        // Server registers winz with the WRONG public key, so verification fails.
        let server_kp = Arc::new(NodeKeypair::generate("ird"));
        let client_kp = NodeKeypair::generate("winz");
        let impostor_kp = NodeKeypair::generate("winz");

        let mut server_peers = HashMap::new();
        server_peers.insert(
            "winz".to_string(),
            PeerConfig {
                dept_id: "winz".into(),
                addr: "127.0.0.1:1".parse().unwrap(),
                public_key_b64: impostor_kp.public_key_b64(),
            },
        );

        let state = InboundState {
            keypair: server_kp.clone(),
            peers: Arc::new(server_peers),
            handler: echo_handler(),
            replay_protection: Arc::new(tokio::sync::Mutex::new(ReplayProtection::new(
                REPLAY_WINDOW_SIZE,
            ))),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: Some(Arc::new(
                RevocationClient::new(
                    "http://127.0.0.1:1",
                    "test-key",
                    Duration::from_secs(5),
                    Duration::from_secs(60),
                )
                .unwrap(),
            )),
        };
        let addr = spawn_server(state).await;

        let peer = PeerConfig {
            dept_id: "ird".into(),
            addr,
            public_key_b64: server_kp.public_key_b64(),
        };

        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let transport = HttpMockTransport::new();
        let err = transport.send(&peer, env).await.unwrap_err();
        assert!(matches!(err, FederationError::Transport(_)));
    }
}
