//! QUIC transport for federation (Phase 2).
//!
//! Exchanges [`FederationEnvelope`] messages over QUIC (UDP) with the same
//! Ed25519 signature verification as the Phase 1 HTTP mock transport
//! ([`crate::transport`]). It is enabled by the `quic` feature.
//!
//! Wire contract (identical to the HTTP transport):
//! - A node accepts inbound envelopes on a QUIC endpoint, each arriving as a
//!   JSON `FederationEnvelope` on a bidirectional stream.
//! - The receiver verifies the sender's Ed25519 signature against the peer's
//!   registered public key, dispatches to the node's handler, signs the
//!   response envelope, and returns it on the same stream.
//!
//! The server presents a throwaway self-signed certificate; the client pins
//! that exact certificate in its root store. Real authentication, however, is
//! delegated to the application layer: every envelope is verified against the
//! peer's registered Ed25519 public key — matching the HTTP mock transport.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use gov_federation_core::{FederationEnvelope, FederationError};
use quinn::{ClientConfig, Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tracing::{debug, info, warn};

use crate::transport::{InboundState, verifying_key_from_b64};

/// Upper bound on a single envelope payload (defence against unbounded reads).
const MAX_ENVELOPE_BYTES: usize = 16 * 1024 * 1024;

/// Generate a throwaway self-signed certificate for the QUIC server.
fn self_signed_cert() -> anyhow::Result<(
    Vec<CertificateDer<'static>>,
    PrivateKeyDer<'static>,
    Vec<u8>,
)> {
    let key = rcgen::KeyPair::generate().context("generate key pair")?;
    let params = rcgen::CertificateParams::new(vec!["localhost".to_string()])
        .context("build cert params")?;
    let cert = params.self_signed(&key).context("self-sign certificate")?;
    let cert_der = cert.der().clone();
    let key_der = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key.serialize_der()));
    Ok((vec![cert_der.clone()], key_der, cert_der.as_ref().to_vec()))
}

/// Build a QUIC server endpoint listening on `addr` with a self-signed cert.
/// Returns the endpoint and the DER of its certificate, which the client
/// must pin in its root store.
pub fn server_endpoint(addr: SocketAddr) -> anyhow::Result<(Endpoint, Vec<u8>)> {
    let (cert_chain, key, cert_der) = self_signed_cert()?;
    let server_config =
        ServerConfig::with_single_cert(cert_chain, key).context("build QUIC server config")?;
    let endpoint =
        Endpoint::server(server_config, addr).with_context(|| format!("bind QUIC on {addr}"))?;
    Ok((endpoint, cert_der))
}

/// Build a QUIC client endpoint that trusts exactly `server_cert_der`.
pub fn client_endpoint(server_cert_der: &[u8]) -> anyhow::Result<Endpoint> {
    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(CertificateDer::from(server_cert_der.to_vec()))
        .context("add pinned server certificate")?;

    let client_config = ClientConfig::with_root_certificates(Arc::new(roots))
        .map_err(|e| anyhow::anyhow!("build QUIC client config: {e}"))?;

    let mut endpoint =
        Endpoint::client("0.0.0.0:0".parse()?).context("bind QUIC client endpoint")?;
    endpoint.set_default_client_config(client_config);
    Ok(endpoint)
}

/// QUIC client transport: sign-and-send an envelope, await the signed reply.
pub struct QuicTransport {
    client: Endpoint,
}

impl QuicTransport {
    pub fn new(server_cert_der: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            client: client_endpoint(server_cert_der)?,
        })
    }

    /// Send `envelope` to the QUIC server at `peer_addr`, awaiting its
    /// signed response. The response signature is verified against
    /// `peer_public_key_b64` before it is returned.
    pub async fn send(
        &self,
        peer_addr: SocketAddr,
        peer_public_key_b64: &str,
        envelope: FederationEnvelope,
    ) -> Result<FederationEnvelope, FederationError> {
        let conn = self
            .client
            .connect(peer_addr, "localhost")
            .map_err(|e| FederationError::Transport(e.to_string()))?
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let (mut send, mut recv) = conn
            .open_bi()
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let bytes =
            serde_json::to_vec(&envelope).map_err(|e| FederationError::Transport(e.to_string()))?;
        send.write_all(&bytes)
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;
        send.finish()
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let resp_bytes = recv
            .read_to_end(MAX_ENVELOPE_BYTES)
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let response: FederationEnvelope = serde_json::from_slice(&resp_bytes)
            .map_err(|e| FederationError::Transport(e.to_string()))?;
        let peer_key = verifying_key_from_b64(peer_public_key_b64)?;
        response.verify(&peer_key)?;
        Ok(response)
    }
}

/// Run the QUIC server accept loop, dispatching each verified envelope to
/// `state.handler` and signing the response. Returns when the endpoint stops
/// accepting connections.
pub async fn serve_quic(endpoint: Endpoint, state: InboundState) -> anyhow::Result<()> {
    if let Ok(addr) = endpoint.local_addr() {
        info!(listen = %addr, "QUIC federation server listening");
    }
    while let Some(incoming) = endpoint.accept().await {
        let state = state.clone();
        tokio::spawn(async move {
            match incoming.accept() {
                Ok(connecting) => match connecting.await {
                    Ok(conn) => {
                        if let Err(e) = handle_connection(conn, state).await {
                            warn!(error = %e, "QUIC connection error");
                        }
                    }
                    Err(e) => warn!(error = %e, "QUIC connection failed"),
                },
                Err(e) => warn!(error = %e, "QUIC incoming accept failed"),
            }
        });
    }
    Ok(())
}

async fn handle_connection(
    conn: quinn::Connection,
    state: InboundState,
) -> Result<(), FederationError> {
    loop {
        let (mut send, mut recv) = conn
            .accept_bi()
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let bytes = recv
            .read_to_end(MAX_ENVELOPE_BYTES)
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let envelope: FederationEnvelope = serde_json::from_slice(&bytes)
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        let peer = state.peers.get(&envelope.from_dept_id).ok_or_else(|| {
            FederationError::Transport(format!("unknown peer: {}", envelope.from_dept_id))
        })?;
        let sender_key = verifying_key_from_b64(&peer.public_key_b64)
            .map_err(|e| FederationError::Crypto(e.to_string()))?;
        envelope
            .verify(&sender_key)
            .map_err(|e| FederationError::Transport(e.to_string()))?;

        debug!(
            from = %envelope.from_dept_id,
            to = %envelope.to_dept_id,
            message_id = %envelope.message_id,
            "inbound QUIC envelope verified"
        );

        let mut response = (state.handler)(envelope)
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;
        response.sign(&state.keypair.signing_key);

        let resp_bytes =
            serde_json::to_vec(&response).map_err(|e| FederationError::Transport(e.to_string()))?;
        send.write_all(&resp_bytes)
            .await
            .map_err(|e| FederationError::Transport(e.to_string()))?;
        send.finish()
            .map_err(|e| FederationError::Transport(e.to_string()))?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;

    use gov_federation_core::{FederationMessageType, NodeKeypair};
    use uuid::Uuid;

    use crate::config::PeerConfig;
    use crate::transport::EnvelopeHandler;

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

    #[tokio::test]
    async fn quic_round_trip() {
        // Server = ird, client = winz.
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

        let (server, server_cert_der) = server_endpoint("127.0.0.1:0".parse().unwrap()).unwrap();
        let addr = server.local_addr().unwrap();
        tokio::spawn(async move {
            let _ = serve_quic(server, state).await;
        });
        // Give the accept loop a moment to come up.
        tokio::time::sleep(Duration::from_millis(150)).await;

        let peer = PeerConfig {
            dept_id: "ird".into(),
            addr,
            public_key_b64: server_kp.public_key_b64(),
        };

        let transport = QuicTransport::new(&server_cert_der).unwrap();
        let mut env =
            FederationEnvelope::new_request("winz", "ird", vec![], "payload".into(), String::new());
        env.sign(&client_kp.signing_key);

        let resp = transport
            .send(addr, &peer.public_key_b64, env)
            .await
            .unwrap();

        assert_eq!(resp.message_type, FederationMessageType::DataResponse);
        assert_eq!(resp.from_dept_id, "ird");
        assert_eq!(resp.to_dept_id, "winz");
        assert!(!resp.signature.is_empty());
    }
}
