use std::collections::HashMap;
#[cfg(feature = "quic")]
use std::net::SocketAddr;
use std::sync::Arc;
#[cfg(feature = "quic")]
use std::sync::OnceLock;

use crate::config::{FederationNodeConfig, PeerConfig};
use crate::transport::{self, EnvelopeHandler, HttpMockTransport, InboundState};
use gov_federation_core::{
    FederationEnvelope, FederationError, FederationMessageType, NodeKeypair,
};
use tracing::info;
use uuid::Uuid;

pub struct FederationNode {
    config: FederationNodeConfig,
    keypair: Arc<NodeKeypair>,
    peers: Arc<HashMap<String, PeerConfig>>,
    handler: EnvelopeHandler,
    transport: HttpMockTransport,
    #[cfg(feature = "quic")]
    quic_server_cert: OnceLock<Vec<u8>>,
}

impl FederationNode {
    pub fn new(config: FederationNodeConfig) -> Self {
        let keypair = Arc::new(NodeKeypair::generate(&config.dept_id));
        info!(
            dept_id = %config.dept_id,
            public_key = %keypair.public_key_b64(),
            peer_count = config.peers.len(),
            "Federation node initialised"
        );

        let peers = Arc::new(
            config
                .peers
                .iter()
                .cloned()
                .map(|p| (p.dept_id.clone(), p))
                .collect::<HashMap<_, _>>(),
        );

        Self {
            config,
            keypair,
            peers,
            handler: default_handler(),
            transport: HttpMockTransport::new(),
            #[cfg(feature = "quic")]
            quic_server_cert: OnceLock::new(),
        }
    }

    /// Replace the inbound envelope handler.
    ///
    /// The default handler acknowledges each request with an empty
    /// `DataResponse`. Real dept nodes install a handler that performs consent
    /// checks and returns the requested data.
    pub fn with_handler(mut self, handler: EnvelopeHandler) -> Self {
        self.handler = handler;
        self
    }

    pub fn dept_id(&self) -> &str {
        &self.config.dept_id
    }

    pub fn public_key_b64(&self) -> String {
        self.keypair.public_key_b64()
    }

    /// Start the HTTP mock transport server (Phase 1).
    ///
    /// The QUIC transport is Phase 2; see the crate roadmap.
    pub async fn start(&self) -> anyhow::Result<()> {
        let state = InboundState {
            keypair: self.keypair.clone(),
            peers: self.peers.clone(),
            handler: self.handler.clone(),
        };
        let app = transport::router(state);

        let listener = tokio::net::TcpListener::bind(self.config.listen_addr).await?;
        info!(
            listen = %self.config.listen_addr,
            "Federation node listening (HTTP mock transport — Phase 1)"
        );
        axum::serve(listener, app).await?;
        Ok(())
    }

    /// Sign and send an envelope to a peer department, awaiting its response.
    ///
    /// The `from`/`to` fields are set from this node and the target peer before
    /// signing, so callers only need to provide the message body.
    pub async fn send(
        &self,
        to_dept_id: &str,
        mut envelope: FederationEnvelope,
    ) -> Result<FederationEnvelope, FederationError> {
        let peer = self
            .peers
            .get(to_dept_id)
            .ok_or_else(|| FederationError::UnknownDepartment(to_dept_id.to_string()))?;

        envelope.from_dept_id = self.config.dept_id.clone();
        envelope.to_dept_id = to_dept_id.to_string();
        envelope.sign(&self.keypair.signing_key);

        self.transport.send(peer, envelope).await
    }

    /// Start a QUIC server (Phase 2 transport) on `TPT__GOV__FEDERATION_QUIC_LISTEN`
    /// (default `0.0.0.0:7001`). The HTTP mock transport ([`Self::start`]) can run
    /// alongside it; both dispatch to the same handler and verify envelope
    /// signatures against the registered peer keys.
    #[cfg(feature = "quic")]
    pub async fn start_quic(&self) -> anyhow::Result<()> {
        let listen: SocketAddr = std::env::var("TPT__GOV__FEDERATION_QUIC_LISTEN")
            .unwrap_or_else(|_| "0.0.0.0:7001".into())
            .parse()?;

        let (endpoint, cert_der) = crate::quic::server_endpoint(listen)?;
        let _ = self.quic_server_cert.set(cert_der);
        let state = crate::transport::InboundState {
            keypair: self.keypair.clone(),
            peers: self.peers.clone(),
            handler: self.handler.clone(),
        };

        tokio::spawn(async move {
            if let Err(e) = crate::quic::serve_quic(endpoint, state).await {
                tracing::error!(error = %e, "QUIC federation server stopped");
            }
        });
        Ok(())
    }

    /// Sign and send an envelope to a peer over QUIC, awaiting its signed
    /// response. `quic_addr` is the peer's QUIC listener address; its Ed25519
    /// public key is looked up from the node's peer table for response
    /// verification.
    #[cfg(feature = "quic")]
    pub async fn send_quic(
        &self,
        quic_addr: SocketAddr,
        mut envelope: FederationEnvelope,
    ) -> anyhow::Result<FederationEnvelope> {
        let peer = self
            .peers
            .get(&envelope.to_dept_id)
            .ok_or_else(|| FederationError::UnknownDepartment(envelope.to_dept_id.clone()))?;

        envelope.from_dept_id = self.config.dept_id.clone();
        envelope.to_dept_id = peer.dept_id.clone();
        envelope.sign(&self.keypair.signing_key);

        let cert_der = self
            .quic_server_cert
            .get()
            .ok_or_else(|| anyhow::anyhow!("QUIC server not started on this node"))?;
        let transport = crate::quic::QuicTransport::new(cert_der)?;
        Ok(transport
            .send(quic_addr, &peer.public_key_b64, envelope)
            .await?)
    }
}

/// Default inbound handler: acknowledges a request with an empty `DataResponse`
/// correlated to the incoming message. Suitable for Phase 1 local development.
pub(crate) fn default_handler() -> EnvelopeHandler {
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
