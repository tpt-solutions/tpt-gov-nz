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

impl FederationNodeConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let dept_id = std::env::var("TPT__GOV__DEPT_ID")?;
        let listen_addr: SocketAddr = std::env::var("TPT__GOV__FEDERATION_LISTEN")
            .unwrap_or_else(|_| "0.0.0.0:7000".into())
            .parse()?;

        Ok(Self { dept_id, listen_addr, peers: vec![] })
    }
}
