use std::collections::HashMap;
#[cfg(feature = "quic")]
use std::net::SocketAddr;
use std::sync::Arc;
#[cfg(feature = "quic")]
use std::sync::OnceLock;
use std::time::Duration;

use crate::config::{FederationNodeConfig, PeerConfig};
use crate::transport::{
    self, DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS, DEFAULT_FEDERATION_RATE_LIMIT_RPM,
    EnvelopeHandler, FederationRateLimiter, HttpMockTransport, InboundState, REPLAY_WINDOW_SIZE,
};
use gov_consent::RevocationClient;
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
    /// Build a node with an ephemeral keypair. Intended for tests and local
    /// experiments; production nodes should use [`Self::try_new`] so their
    /// identity survives restarts.
    pub fn new(config: FederationNodeConfig) -> Self {
        let keypair = NodeKeypair::generate(&config.dept_id);
        Self::with_keypair(config, keypair)
    }

    /// Build a node whose identity is persistent.
    ///
    /// The keypair is taken from, in order:
    /// 1. `TPT__GOV__FEDERATION_SECRET_KEY` (base64 Ed25519 secret);
    /// 2. the key file at `TPT__GOV__FEDERATION_KEY_FILE`, if it exists;
    /// 3. otherwise a new key is generated and written to that file, so the
    ///    node keeps the same identity on the next start.
    ///
    /// Without either variable the node falls back to an ephemeral key and logs
    /// a warning, because peers' registered public keys would stop matching.
    pub fn try_new(config: FederationNodeConfig) -> anyhow::Result<Self> {
        let keypair = load_or_create_keypair(&config.dept_id)?;
        Ok(Self::with_keypair(config, keypair))
    }

    fn with_keypair(config: FederationNodeConfig, keypair: NodeKeypair) -> Self {
        let keypair = Arc::new(keypair);
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
            replay_protection: Arc::new(tokio::sync::Mutex::new(
                crate::transport::ReplayProtection::new(REPLAY_WINDOW_SIZE),
            )),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: Some(Arc::new(RevocationClient::from_env()?)),
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
            replay_protection: Arc::new(tokio::sync::Mutex::new(
                crate::transport::ReplayProtection::new(REPLAY_WINDOW_SIZE),
            )),
            federation_rate_limiter: Arc::new(FederationRateLimiter::new(
                DEFAULT_FEDERATION_RATE_LIMIT_RPM,
                Duration::from_secs(60),
                DEFAULT_FEDERATION_RATE_LIMIT_MAX_PEERS,
            )),
            revocation_client: Some(Arc::new(RevocationClient::from_env()?)),
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
        let transport = crate::quic::QuicTransport::new(cert_der.to_vec())?;
        let response = transport
            .send(quic_addr, &peer.public_key_b64, envelope)
            .await?;
        Ok(response)
    }
}

/// Environment variable holding the node's base64 Ed25519 secret key.
pub const FEDERATION_SECRET_KEY_ENV: &str = "TPT__GOV__FEDERATION_SECRET_KEY";
/// Environment variable naming the file where the node's secret key is kept.
pub const FEDERATION_KEY_FILE_ENV: &str = "TPT__GOV__FEDERATION_KEY_FILE";

/// Resolve the node keypair from the environment according to
/// [`FederationNode::try_new`].
fn load_or_create_keypair(dept_id: &str) -> anyhow::Result<NodeKeypair> {
    resolve_keypair(
        dept_id,
        std::env::var(FEDERATION_SECRET_KEY_ENV).ok(),
        std::env::var(FEDERATION_KEY_FILE_ENV)
            .ok()
            .map(std::path::PathBuf::from),
    )
}

/// Pure key resolution: explicit secret first, then the key file (created if
/// missing), then an ephemeral key with a warning.
fn resolve_keypair(
    dept_id: &str,
    secret: Option<String>,
    key_file: Option<std::path::PathBuf>,
) -> anyhow::Result<NodeKeypair> {
    if let Some(secret) = secret {
        return NodeKeypair::from_secret_b64_with_dept_id(&secret, dept_id)
            .map_err(|e| anyhow::anyhow!("invalid {FEDERATION_SECRET_KEY_ENV}: {e}"));
    }

    let Some(path) = key_file else {
        tracing::warn!(
            dept_id,
            "no federation key configured; using an ephemeral key. Set {} or {} for a stable identity.",
            FEDERATION_SECRET_KEY_ENV,
            FEDERATION_KEY_FILE_ENV
        );
        return Ok(NodeKeypair::generate(dept_id));
    };

    match std::fs::read_to_string(&path) {
        Ok(secret) => NodeKeypair::from_secret_b64_with_dept_id(&secret, dept_id)
            .map_err(|e| anyhow::anyhow!("invalid federation key file {}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let keypair = NodeKeypair::generate(dept_id);
            write_secret_file(&path, &keypair.secret_key_b64())?;
            info!(path = %path.display(), "created new federation key file");
            Ok(keypair)
        }
        Err(e) => Err(anyhow::anyhow!(
            "cannot read federation key file {}: {e}",
            path.display()
        )),
    }
}

/// Write a secret without overwriting an existing file, restricting it to the
/// owner on Unix.
fn write_secret_file(path: &std::path::Path, contents: &str) -> anyhow::Result<()> {
    use std::io::Write;

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).map_err(|e| {
        anyhow::anyhow!("cannot create federation key file {}: {e}", path.display())
    })?;
    file.write_all(contents.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
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

#[cfg(test)]
mod keypair_tests {
    use super::*;

    fn temp_key_path(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("tpt-fed-key-{}-{}", name, uuid::Uuid::new_v4()));
        dir.join("node.key")
    }

    #[test]
    fn key_file_is_created_once_and_reloaded_with_same_identity() {
        let path = temp_key_path("reload");
        let first = resolve_keypair("ird", None, Some(path.clone())).unwrap();
        assert!(path.exists());

        let second = resolve_keypair("ird", None, Some(path.clone())).unwrap();
        assert_eq!(first.public_key_b64(), second.public_key_b64());
        assert_eq!(second.dept_id, "ird");

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn explicit_secret_takes_precedence_over_key_file() {
        let secret = NodeKeypair::generate("x").secret_key_b64();
        let expected = NodeKeypair::from_secret_b64(&secret)
            .unwrap()
            .public_key_b64();
        let path = temp_key_path("precedence");

        let kp = resolve_keypair("winz", Some(secret), Some(path.clone())).unwrap();
        assert_eq!(kp.public_key_b64(), expected);
        assert_eq!(kp.dept_id, "winz");
        assert!(
            !path.exists(),
            "no key file should be written when a secret is given"
        );
    }

    #[test]
    fn invalid_secret_fails_closed() {
        assert!(resolve_keypair("ird", Some("not-a-key".into()), None).is_err());
    }

    #[test]
    fn write_secret_file_never_overwrites() {
        let path = temp_key_path("no-overwrite");
        write_secret_file(&path, "first").unwrap();
        assert!(write_secret_file(&path, "second").is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap().trim(), "first");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }
}
