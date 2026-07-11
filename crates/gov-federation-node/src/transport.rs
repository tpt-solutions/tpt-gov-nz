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

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use ed25519_dalek::VerifyingKey;
use gov_federation_core::{FederationEnvelope, FederationError, NodeKeypair};
use tracing::{debug, warn};

use crate::config::PeerConfig;

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
}

/// Build the axum router exposing the inbound envelope endpoint.
pub fn router(state: InboundState) -> Router {
    Router::new()
        .route(ENVELOPE_PATH, post(receive_envelope))
        .with_state(state)
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
