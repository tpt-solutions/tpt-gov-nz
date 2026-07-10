use std::collections::HashMap;
use std::sync::Arc;

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
