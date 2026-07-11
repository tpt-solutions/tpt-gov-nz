use std::path::PathBuf;

use crate::{
    error::IngestError,
    transport::legacy::LegacyConfig,
};

#[derive(Debug, Clone)]
pub enum TransportKind {
    Mock,
    Legacy,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub transport: TransportKind,
    pub mock_fixture: PathBuf,
    pub legacy: LegacyConfig,
    pub interval_secs: u64,
    pub run_once: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, IngestError> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| IngestError::Config("DATABASE_URL must be set".into()))?;

        let transport = match std::env::var("RETIREMENT_TRANSPORT")
            .unwrap_or_else(|_| "mock".into())
            .to_lowercase()
            .as_str()
        {
            "legacy" => TransportKind::Legacy,
            _ => TransportKind::Mock,
        };

        let mock_fixture = PathBuf::from(
            std::env::var("RETIREMENT_MOCK_FIXTURE")
                .unwrap_or_else(|_| "fixtures/retirement_batch.json".into()),
        );

        let legacy = LegacyConfig {
            host: std::env::var("RETIREMENT_LEGACY_HOST")
                .unwrap_or_else(|_| "sftp.retirement.govt.nz".into()),
            port: std::env::var("RETIREMENT_LEGACY_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(22),
            username: std::env::var("RETIREMENT_LEGACY_USER").unwrap_or_default(),
            private_key_path: PathBuf::from(
                std::env::var("RETIREMENT_LEGACY_KEY").unwrap_or_default(),
            ),
            remote_path: std::env::var("RETIREMENT_LEGACY_REMOTE")
                .unwrap_or_else(|_| "/inbound/retirement_batch.json".into()),
        };

        let interval_secs = std::env::var("RETIREMENT_INGEST_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);

        let run_once = std::env::var("RETIREMENT_RUN_ONCE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            transport,
            mock_fixture,
            legacy,
            interval_secs,
            run_once,
        })
    }
}
