//! QUIC transport for federation nodes (Phase 2).
//!
//! This module provides QUIC-based transport for `FederationEnvelope` messages
//! using the `quinn` crate with self-signed certificates and Ed25519 envelope
//! authentication. The QUIC transport is gated behind the `quic` feature flag.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine;
use gov_federation_core::{FederationEnvelope, NodeKeypair};
use quinn::{ClientConfig, Endpoint, ServerConfig, TransportConfig};
use rcgen::{CertificateParams, DistinguishedName, DnType, Ia5String, KeyPair, SanType};
use rustls::RootCertStore;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tracing::{debug, error, info, warn};

use crate::transport::InboundState;

/// QUIC ALPN protocol identifier for tpt-gov-nz federation.
pub const QUIC_ALPN: &[u8] = b"tpt-gov-nz/1";

/// Maximum message size for QUIC streams (1 MiB).
const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// QUIC idle timeout.
const QUIC_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

/// Generate a self-signed certificate for QUIC server.
///
/// Returns (certificate DER, private key DER).
fn generate_self_signed_cert(dept_id: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut params = CertificateParams::default();
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, format!("tpt-gov-nz-{}", dept_id));
    params
        .distinguished_name
        .push(DnType::OrganizationName, "tpt-gov-nz");
    params.subject_alt_names = vec![SanType::DnsName(Ia5String::try_from(format!(
        "{}.federation.tpt.gov.nz",
        dept_id
    ))?)];
    params.key_usages = vec![
        rcgen::KeyUsagePurpose::DigitalSignature,
        rcgen::KeyUsagePurpose::KeyEncipherment,
    ];
    params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];

    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    let cert_der = cert.der().to_vec();
    let key_der = key_pair.serialize_der();

    Ok((cert_der, key_der))
}

/// Create a QUIC server endpoint with a self-signed certificate.
///
/// Returns the endpoint and the certificate DER for clients to verify.
pub fn server_endpoint(listen: SocketAddr) -> Result<(Endpoint, Vec<u8>)> {
    let (cert_der, key_der) = generate_self_signed_cert("federation")?;

    let cert = CertificateDer::from(cert_der.clone());
    let key = PrivateKeyDer::from(PrivatePkcs8KeyDer::from(key_der.clone()));

    let mut server_config = ServerConfig::with_single_cert(vec![cert], key)?;
    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(QUIC_IDLE_TIMEOUT.try_into()?));
    transport_config.keep_alive_interval(Some(Duration::from_secs(10)));
    server_config.transport_config(Arc::new(transport_config));

    let mut endpoint = Endpoint::server(server_config, listen)?;
    endpoint.set_default_client_config(client_config_from_cert(&cert_der)?);

    Ok((endpoint, cert_der))
}

/// Create a QUIC client endpoint that trusts a specific certificate.
pub fn client_endpoint(server_cert_der: &[u8]) -> Result<Endpoint> {
    let client_config = client_config_from_cert(server_cert_der)?;
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);
    Ok(endpoint)
}

/// Create a QUIC client config that trusts a specific certificate.
fn client_config_from_cert(cert_der: &[u8]) -> Result<ClientConfig> {
    let cert = CertificateDer::from(cert_der.to_vec());
    let mut roots = RootCertStore::empty();
    roots.add(cert)?;
    let client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    let client_config = quinn::crypto::rustls::QuicClientConfig::try_from(Arc::new(client_crypto))?;
    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(QUIC_IDLE_TIMEOUT.try_into()?));
    transport_config.keep_alive_interval(Some(Duration::from_secs(10)));
    let mut client_config = ClientConfig::new(Arc::new(client_config));
    client_config.transport_config(Arc::new(transport_config));
    Ok(client_config)
}

/// QUIC client transport for sending envelopes to peers.
pub struct QuicTransport {
    endpoint: Endpoint,
}

impl QuicTransport {
    /// Create a new QUIC transport with the server's certificate for verification.
    pub fn new(server_cert_der: Vec<u8>) -> Result<Self> {
        let endpoint = client_endpoint(&server_cert_der)?;
        Ok(Self { endpoint })
    }

    /// Send an envelope to a peer over QUIC and await the response.
    pub async fn send(
        &self,
        peer_addr: SocketAddr,
        peer_public_key_b64: &str,
        mut envelope: FederationEnvelope,
    ) -> Result<FederationEnvelope> {
        // Connect to peer
        let connection = self
            .endpoint
            .connect(peer_addr, "federation.tpt.gov.nz")?
            .await
            .context("QUIC connection failed")?;

        // Open a bidirectional stream
        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

        // Serialize and send envelope
        let payload = serde_json::to_vec(&envelope)?;
        if payload.len() > MAX_MESSAGE_SIZE {
            anyhow::bail!("envelope too large: {} bytes", payload.len());
        }

        // Send length prefix (4 bytes big-endian) + payload
        let len_bytes = (payload.len() as u32).to_be_bytes();
        send_stream.write_all(&len_bytes).await?;
        send_stream.write_all(&payload).await?;
        send_stream.finish()?;

        // Read response
        let mut len_buf = [0u8; 4];
        recv_stream.read_exact(&mut len_buf).await?;
        let resp_len = u32::from_be_bytes(len_buf) as usize;
        if resp_len > MAX_MESSAGE_SIZE {
            anyhow::bail!("response too large: {} bytes", resp_len);
        }

        let mut resp_buf = vec![0u8; resp_len];
        recv_stream.read_exact(&mut resp_buf).await?;

        let response: FederationEnvelope = serde_json::from_slice(&resp_buf)?;

        // Verify response signature against peer's public key
        let peer_pubkey = base64::engine::general_purpose::STANDARD
            .decode(peer_public_key_b64)
            .context("invalid peer public key base64")?;
        let peer_key = ed25519_dalek::VerifyingKey::from_bytes(
            &peer_pubkey
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid key length"))?,
        )?;
        response
            .verify(&peer_key)
            .context("response signature verification failed")?;

        // Verify response is for our request
        if response.correlation_id != Some(envelope.message_id) {
            anyhow::bail!("response correlation_id mismatch");
        }

        Ok(response)
    }
}
/// Serve QUIC connections and handle incoming envelopes.
pub async fn serve_quic(endpoint: Endpoint, state: InboundState) -> Result<()> {
    info!("QUIC federation server started");
    loop {
        let connecting = match endpoint.accept().await {
            Some(c) => c,
            None => break,
        };

        let state = state.clone();
        tokio::spawn(async move {
            let connection = match connecting.await {
                Ok(c) => c,
                Err(e) => {
                    warn!(error = %e, "QUIC connection failed");
                    return;
                }
            };

            loop {
                let (mut send_stream, mut recv_stream) = match connection.accept_bi().await {
                    Ok(s) => s,
                    Err(e) => {
                        if e.to_string().contains("ApplicationClosed") {
                            debug!("QUIC stream closed by peer");
                        } else {
                            warn!(error = %e, "QUIC accept_bi failed");
                        }
                        return;
                    }
                };

                let state = state.clone();
                tokio::spawn(async move {
                    // Read length prefix
                    let mut len_buf = [0u8; 4];
                    if recv_stream.read_exact(&mut len_buf).await.is_err() {
                        return;
                    }
                    let msg_len = u32::from_be_bytes(len_buf) as usize;
                    if msg_len > MAX_MESSAGE_SIZE {
                        warn!(msg_len, "QUIC message too large");
                        return;
                    }

                    // Read payload
                    let mut buf = vec![0u8; msg_len];
                    if recv_stream.read_exact(&mut buf).await.is_err() {
                        return;
                    }

                    // Parse envelope
                    let envelope: FederationEnvelope = match serde_json::from_slice(&buf) {
                        Ok(e) => e,
                        Err(e) => {
                            warn!(error = %e, "Failed to parse QUIC envelope");
                            return;
                        }
                    };

                    // Verify envelope signature
                    let from_dept = &envelope.from_dept_id;
                    let peer_config = state.peers.get(from_dept);
                    if peer_config.is_none() {
                        warn!(from_dept = %from_dept, "Unknown peer department");
                        return;
                    }
                    let peer_config = peer_config.unwrap();

                    let peer_pubkey = match base64::engine::general_purpose::STANDARD
                        .decode(&peer_config.public_key_b64)
                    {
                        Ok(k) => k,
                        Err(e) => {
                            warn!(error = %e, "Invalid peer public key");
                            return;
                        }
                    };
                    let peer_key_bytes: [u8; 32] = match peer_pubkey.try_into() {
                        Ok(k) => k,
                        Err(_) => {
                            warn!("Invalid peer public key length");
                            return;
                        }
                    };
                    let peer_key = match ed25519_dalek::VerifyingKey::from_bytes(&peer_key_bytes) {
                        Ok(k) => k,
                        Err(e) => {
                            warn!(error = %e, "Invalid peer verifying key");
                            return;
                        }
                    };

                    if envelope.verify(&peer_key).is_err() {
                        warn!(from_dept = %from_dept, "QUIC envelope signature verification failed");
                        return;
                    }
                    // Check timestamp freshness
                    let now_ms = chrono::Utc::now().timestamp_millis();
                    if (now_ms - envelope.timestamp_ms).abs()
                        > crate::transport::MAX_TIMESTAMP_SKEW_SECS * 1000
                    {
                        warn!(from_dept = %from_dept, "QUIC envelope timestamp skew exceeded");
                        return;
                    }

                    // Replay protection
                    if state
                        .replay_protection
                        .lock()
                        .await
                        .check_and_insert(envelope.message_id)
                        .is_err()
                    {
                        warn!(from_dept = %from_dept, msg_id = %envelope.message_id, "QUIC replay detected");
                        return;
                    }

                    // Rate limiting
                    if state.federation_rate_limiter.check(from_dept).is_err() {
                        warn!(from_dept = %from_dept, "QUIC rate limit exceeded");
                        return;
                    }

                    // Revocation check
                    if let Some(revocation) = &state.revocation_client {
                        if !envelope.consent_grant_ids.is_empty() {
                            let grant_ids: Vec<_> =
                                envelope.consent_grant_ids.iter().copied().collect();
                            if !grant_ids.is_empty() {
                                if revocation.ensure_active(&grant_ids).await.is_err() {
                                    warn!(from_dept = %from_dept, "QUIC grant revoked or unknown");
                                    return;
                                }
                            }
                        }
                    }

                    // Dispatch to handler
                    let response = match (state.handler)(envelope).await {
                        Ok(r) => r,
                        Err(e) => {
                            error!(error = %e, "QUIC handler error");
                            return;
                        }
                    };

                    // Sign response
                    let mut signed_response = response;
                    signed_response.from_dept_id = state.keypair.dept_id.to_string();
                    signed_response.sign(&state.keypair.signing_key);

                    // Send response
                    let resp_payload = match serde_json::to_vec(&signed_response) {
                        Ok(p) => p,
                        Err(e) => {
                            error!(error = %e, "Failed to serialize QUIC response");
                            return;
                        }
                    };
                    if resp_payload.len() > MAX_MESSAGE_SIZE {
                        warn!("QUIC response too large");
                        return;
                    }

                    let resp_len = (resp_payload.len() as u32).to_be_bytes();
                    if send_stream.write_all(&resp_len).await.is_err() {
                        return;
                    }
                    if send_stream.write_all(&resp_payload).await.is_err() {
                        return;
                    }
                    let _ = send_stream.finish();
                });
            }
        });
    }
    Ok(())
}

#[cfg(all(test, feature = "quic"))]
mod tests {
    use super::*;
    use gov_federation_core::NodeKeypair;
    use std::net::SocketAddr;
    use std::sync::Once;

    static CRYPTO_PROVIDER: Once = Once::new();

    fn install_crypto_provider() {
        CRYPTO_PROVIDER.call_once(|| {
            let _ = rustls::crypto::ring::default_provider().install_default();
        });
    }

    #[tokio::test]
    async fn test_server_endpoint_creation() {
        install_crypto_provider();
        let listen: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let (endpoint, cert_der) = server_endpoint(listen).unwrap();
        assert!(!cert_der.is_empty());
        drop(endpoint);
    }

    #[tokio::test]
    async fn test_client_endpoint_creation() {
        install_crypto_provider();
        let (_endpoint, cert_der) = server_endpoint("127.0.0.1:0".parse().unwrap()).unwrap();
        let endpoint = client_endpoint(&cert_der).unwrap();
        drop(endpoint);
    }

    #[tokio::test]
    async fn test_quic_transport_creation() {
        install_crypto_provider();
        let (_endpoint, cert_der) = server_endpoint("127.0.0.1:0".parse().unwrap()).unwrap();
        let transport = QuicTransport::new(cert_der).unwrap();
        drop(transport);
    }

    #[tokio::test]
    async fn test_self_signed_cert_generation() {
        install_crypto_provider();
        let (cert_der, key_der) = generate_self_signed_cert("test").unwrap();
        assert!(!cert_der.is_empty());
        assert!(!key_der.is_empty());
    }
}
