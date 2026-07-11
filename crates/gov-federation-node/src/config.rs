use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct FederationNodeConfig {
    pub dept_id: String,
    pub listen_addr: SocketAddr,
    pub peers: Vec<PeerConfig>,
}

#[derive(Debug, Clone)]
pub struct PeerConfig {
    pub dept_id: String,
    pub addr: SocketAddr,
    pub public_key_b64: String,
}

impl PeerConfig {
    /// Base HTTP endpoint for the Phase 1 mock transport, derived from `addr`.
    pub fn http_endpoint(&self) -> String {
        format!("http://{}", self.addr)
    }
}

impl FederationNodeConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let dept_id = std::env::var("TPT__GOV__DEPT_ID")?;
        let listen_addr: SocketAddr = std::env::var("TPT__GOV__FEDERATION_LISTEN")
            .unwrap_or_else(|_| "0.0.0.0:7000".into())
            .parse()?;

        let peers = std::env::var("TPT__GOV__FEDERATION_PEERS")
            .ok()
            .map(|raw| Self::parse_peers(&raw))
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            dept_id,
            listen_addr,
            peers,
        })
    }

    /// Parse a peer list from a compact env string.
    ///
    /// Format: comma-separated `dept_id@host:port#base64_public_key` entries, e.g.
    /// `ird@127.0.0.1:7001#abc...,winz@127.0.0.1:7002#def...`
    fn parse_peers(raw: &str) -> anyhow::Result<Vec<PeerConfig>> {
        let mut peers = Vec::new();
        for entry in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (dept_id, rest) = entry
                .split_once('@')
                .ok_or_else(|| anyhow::anyhow!("peer entry missing '@': {entry}"))?;
            let (addr, key) = rest
                .split_once('#')
                .ok_or_else(|| anyhow::anyhow!("peer entry missing '#': {entry}"))?;
            peers.push(PeerConfig {
                dept_id: dept_id.trim().to_string(),
                addr: addr.trim().parse()?,
                public_key_b64: key.trim().to_string(),
            });
        }
        Ok(peers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_peers_multiple_entries() {
        let peers =
            FederationNodeConfig::parse_peers("ird@127.0.0.1:7001#keyA, winz@127.0.0.1:7002#keyB")
                .unwrap();
        assert_eq!(peers.len(), 2);
        assert_eq!(peers[0].dept_id, "ird");
        assert_eq!(peers[0].addr.port(), 7001);
        assert_eq!(peers[0].public_key_b64, "keyA");
        assert_eq!(peers[1].dept_id, "winz");
        assert_eq!(peers[1].http_endpoint(), "http://127.0.0.1:7002");
    }

    #[test]
    fn parse_peers_empty_string() {
        assert!(FederationNodeConfig::parse_peers("").unwrap().is_empty());
    }

    #[test]
    fn parse_peers_rejects_malformed() {
        assert!(FederationNodeConfig::parse_peers("bad-entry").is_err());
    }
}
