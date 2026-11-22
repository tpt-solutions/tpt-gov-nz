// Durable NodeRegistry for federation peer management
//
// The NodeRegistry persists peer information across restarts and supports
// token-based enrolment for new departments joining the federation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::config::PeerConfig;
use anyhow::{Context, Result};
use base64::Engine;
use ed25519_dalek::SigningKey;
use gov_federation_core::NodeKeypair;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;
/// Token for enrolment -- issued by the federation admin to a new department.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrolmentToken {
    pub token_id: Uuid,
    pub dept_id: String,
    pub display_name: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub used: bool,
}

/// Persistent storage for the NodeRegistry.
#[derive(Debug, Serialize, Deserialize, Default)]
struct RegistryStorage {
    peers: HashMap<String, StoredPeer>,
    enrolment_tokens: HashMap<String, EnrolmentToken>,
    this_node: Option<StoredNode>,
}

/// Stored peer configuration (serializable).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredPeer {
    dept_id: String,
    display_name: String,
    addr: String,
    public_key_b64: String,
    enrolled_at: chrono::DateTime<chrono::Utc>,
    last_seen: Option<chrono::DateTime<chrono::Utc>>,
    is_active: bool,
}

/// Stored node configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredNode {
    dept_id: String,
    display_name: String,
    signing_key_b64: String,
    addr: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Durable NodeRegistry that persists to disk.
pub struct NodeRegistry {
    storage_path: PathBuf,
    inner: Arc<RwLock<RegistryStorage>>,
    save_interval: Duration,
}

impl NodeRegistry {
    /// Create a new registry, loading from disk if it exists.
    pub async fn new(storage_path: PathBuf) -> Result<Self> {
        let inner = if storage_path.exists() {
            let data = tokio::fs::read(&storage_path)
                .await
                .with_context(|| format!("reading registry from {}", storage_path.display()))?;
            let storage: RegistryStorage =
                serde_json::from_slice(&data).with_context(|| "deserializing registry")?;
            Arc::new(RwLock::new(storage))
        } else {
            Arc::new(RwLock::new(RegistryStorage::default()))
        };

        let registry = Self {
            storage_path,
            inner,
            save_interval: Duration::from_secs(60),
        };

        let registry_clone = registry.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(registry_clone.save_interval);
            loop {
                interval.tick().await;
                if let Err(e) = registry_clone.save().await {
                    warn!(error = %e, "Failed to save registry");
                }
            }
        });

        Ok(registry)
    }

    /// Save the registry to disk.
    pub async fn save(&self) -> Result<()> {
        let storage = self.inner.read().await;
        let data = serde_json::to_vec_pretty(&*storage).context("serializing registry")?;
        let temp_path = self.storage_path.with_extension("tmp");
        tokio::fs::write(&temp_path, data)
            .await
            .with_context(|| format!("writing temp registry to {}", temp_path.display()))?;
        tokio::fs::rename(&temp_path, &self.storage_path)
            .await
            .with_context(|| {
                format!("renaming temp registry to {}", self.storage_path.display())
            })?;
        Ok(())
    }

    /// Get or create this node's identity.
    pub async fn get_or_create_node(
        &self,
        dept_id: &str,
        display_name: &str,
        addr: &str,
    ) -> Result<NodeKeypair> {
        let mut storage = self.inner.write().await;

        if let Some(node) = &storage.this_node {
            if node.dept_id == dept_id {
                let signing_key = SigningKey::from_bytes(
                    &base64::engine::general_purpose::STANDARD
                        .decode(&node.signing_key_b64)
                        .context("decoding stored signing key")?
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("invalid key length"))?,
                );
                let verifying_key = signing_key.verifying_key();
                return Ok(NodeKeypair {
                    signing_key,
                    verifying_key,
                    dept_id: node.dept_id.clone(),
                });
            }
        }

        let keypair = NodeKeypair::generate(dept_id);
        let signing_key_b64 =
            base64::engine::general_purpose::STANDARD.encode(keypair.signing_key.to_bytes());

        storage.this_node = Some(StoredNode {
            dept_id: dept_id.to_string(),
            display_name: display_name.to_string(),
            signing_key_b64,
            addr: addr.to_string(),
            created_at: chrono::Utc::now(),
        });

        drop(storage);
        self.save().await?;
        Ok(keypair)
    }

    /// Get this node's info.
    pub async fn get_node_info(&self) -> Option<StoredNode> {
        self.inner.read().await.this_node.clone()
    }

    /// Register a new peer (called when enrolment token is used).
    pub async fn register_peer(&self, peer: PeerConfig, display_name: &str) -> Result<()> {
        let mut storage = self.inner.write().await;

        let stored = StoredPeer {
            dept_id: peer.dept_id.clone(),
            display_name: display_name.to_string(),
            addr: peer.addr.to_string(),
            public_key_b64: peer.public_key_b64.clone(),
            enrolled_at: chrono::Utc::now(),
            last_seen: None,
            is_active: true,
        };

        storage.peers.insert(peer.dept_id.clone(), stored);
        drop(storage);
        self.save().await?;
        info!(dept_id = %peer.dept_id, "Peer registered in registry");
        Ok(())
    }

    /// Get a peer by dept_id.
    pub async fn get_peer(&self, dept_id: &str) -> Option<PeerConfig> {
        let storage = self.inner.read().await;
        storage.peers.get(dept_id).map(|p| PeerConfig {
            dept_id: p.dept_id.clone(),
            addr: p.addr.parse().unwrap(),
            public_key_b64: p.public_key_b64.clone(),
        })
    }

    /// Get all active peers.
    pub async fn get_active_peers(&self) -> HashMap<String, PeerConfig> {
        let storage = self.inner.read().await;
        storage
            .peers
            .iter()
            .filter(|(_, p)| p.is_active)
            .map(|(k, p)| {
                (
                    k.clone(),
                    PeerConfig {
                        dept_id: p.dept_id.clone(),
                        addr: p.addr.parse().unwrap(),
                        public_key_b64: p.public_key_b64.clone(),
                    },
                )
            })
            .collect()
    }

    /// Update peer last_seen timestamp.
    pub async fn update_peer_last_seen(&self, dept_id: &str) -> Result<()> {
        let mut storage = self.inner.write().await;
        if let Some(peer) = storage.peers.get_mut(dept_id) {
            peer.last_seen = Some(chrono::Utc::now());
        }
        Ok(())
    }

    /// Create an enrolment token for a new department.
    pub async fn create_enrolment_token(
        &self,
        dept_id: &str,
        display_name: &str,
        ttl_hours: u64,
    ) -> Result<EnrolmentToken> {
        let mut storage = self.inner.write().await;

        let token = EnrolmentToken {
            token_id: Uuid::new_v4(),
            dept_id: dept_id.to_string(),
            display_name: display_name.to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(ttl_hours as i64),
            created_at: chrono::Utc::now(),
            used: false,
        };

        storage
            .enrolment_tokens
            .insert(token.token_id.to_string(), token.clone());
        drop(storage);
        self.save().await?;
        info!(dept_id = %dept_id, token_id = %token.token_id, "Enrolment token created");
        Ok(token)
    }

    /// Validate and consume an enrolment token.
    pub async fn consume_enrolment_token(&self, token_id: Uuid) -> Result<EnrolmentToken> {
        let mut storage = self.inner.write().await;
        let token = storage
            .enrolment_tokens
            .get_mut(&token_id.to_string())
            .ok_or_else(|| anyhow::anyhow!("Enrolment token not found"))?;

        if token.used {
            anyhow::bail!("Enrolment token already used");
        }
        if chrono::Utc::now() > token.expires_at {
            anyhow::bail!("Enrolment token expired");
        }

        token.used = true;
        let consumed = token.clone();
        drop(storage);
        self.save().await?;
        Ok(consumed)
    }

    /// Get all enrolment tokens (for admin UI).
    pub async fn get_enrolment_tokens(&self) -> Vec<EnrolmentToken> {
        self.inner
            .read()
            .await
            .enrolment_tokens
            .values()
            .cloned()
            .collect()
    }

    /// Clean up expired enrolment tokens.
    pub async fn cleanup_expired_tokens(&self) -> Result<()> {
        let mut storage = self.inner.write().await;
        let now = chrono::Utc::now();
        storage
            .enrolment_tokens
            .retain(|_, t| !t.used && t.expires_at > now);
        drop(storage);
        self.save().await?;
        Ok(())
    }
}

impl Clone for NodeRegistry {
    fn clone(&self) -> Self {
        Self {
            storage_path: self.storage_path.clone(),
            inner: self.inner.clone(),
            save_interval: self.save_interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_registry_persists_node() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("registry.json");

        let registry = NodeRegistry::new(path.clone()).await.unwrap();
        let keypair = registry
            .get_or_create_node("ird", "Inland Revenue", "127.0.0.1:7001")
            .await
            .unwrap();
        assert_eq!(keypair.dept_id, "ird");

        let registry2 = NodeRegistry::new(path).await.unwrap();
        let keypair2 = registry2
            .get_or_create_node("ird", "Inland Revenue", "127.0.0.1:7001")
            .await
            .unwrap();
        assert_eq!(keypair.public_key_b64(), keypair2.public_key_b64());
    }

    #[tokio::test]
    async fn test_enrolment_token_lifecycle() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("registry.json");
        let registry = NodeRegistry::new(path).await.unwrap();

        let token = registry
            .create_enrolment_token("newdept", "New Department", 24)
            .await
            .unwrap();
        assert!(!token.used);

        let consumed = registry
            .consume_enrolment_token(token.token_id)
            .await
            .unwrap();
        assert!(consumed.used);

        assert!(
            registry
                .consume_enrolment_token(token.token_id)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_peer_registration() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("registry.json");
        let registry = NodeRegistry::new(path).await.unwrap();

        let keypair = NodeKeypair::generate("winz");
        let peer = PeerConfig {
            dept_id: "winz".to_string(),
            addr: "127.0.0.1:7002".parse().unwrap(),
            public_key_b64: keypair.public_key_b64(),
        };

        registry
            .register_peer(peer.clone(), "Work and Income")
            .await
            .unwrap();
        let retrieved = registry.get_peer("winz").await.unwrap();
        assert_eq!(retrieved.public_key_b64, peer.public_key_b64);
    }
}
