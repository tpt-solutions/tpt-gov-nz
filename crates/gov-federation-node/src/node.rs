use crate::config::FederationNodeConfig;
use gov_federation_core::{FederationEnvelope, NodeKeypair};
use tracing::info;

pub struct FederationNode {
    config: FederationNodeConfig,
    keypair: NodeKeypair,
}

impl FederationNode {
    pub fn new(config: FederationNodeConfig) -> Self {
        let keypair = NodeKeypair::generate(&config.dept_id);
        info!(
            dept_id = %config.dept_id,
            public_key = %keypair.public_key_b64(),
            "Federation node initialised"
        );
        Self { config, keypair }
    }

    pub fn dept_id(&self) -> &str {
        &self.config.dept_id
    }

    pub fn public_key_b64(&self) -> String {
        self.keypair.public_key_b64()
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!(
            listen = %self.config.listen_addr,
            "Federation node listening (QUIC transport — Phase 2)"
        );
        // QUIC listener implementation in Phase 2
        // For Phase 1: HTTP-based mock transport for local development
        Ok(())
    }

    pub async fn send(&self, _envelope: FederationEnvelope) -> anyhow::Result<FederationEnvelope> {
        // Phase 1: stub — returns a mock response
        // Phase 2: QUIC connection to peer, send envelope, await response
        unimplemented!("Federation send — implement in Phase 2")
    }
}
