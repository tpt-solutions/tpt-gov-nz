use std::path::PathBuf;

use crate::{
    error::IngestError,
    transport::sftp::SftpConfig,
};

#[derive(Debug, Clone)]
pub enum TransportKind {
    Mock,
    Sftp,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub transport: TransportKind,
    pub mock_fixture: PathBuf,
    pub sftp: SftpConfig,
    /// Interval between runs, in seconds.
    pub interval_secs: u64,
    /// Run a single pass and exit (useful for demos, tests, and k8s Jobs).
    pub run_once: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, IngestError> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| IngestError::Config("DATABASE_URL must be set".into()))?;

        let transport = match std::env::var("IRD_TRANSPORT")
            .unwrap_or_else(|_| "mock".into())
            .to_lowercase()
            .as_str()
        {
            "sftp" => TransportKind::Sftp,
            _ => TransportKind::Mock,
        };

        let mock_fixture = PathBuf::from(
            std::env::var("IRD_MOCK_FIXTURE")
                .unwrap_or_else(|_| "fixtures/ird_batch.json".into()),
        );

        let sftp = SftpConfig {
            host: std::env::var("IRD_SFTP_HOST").unwrap_or_default(),
            port: std::env::var("IRD_SFTP_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(22),
            username: std::env::var("IRD_SFTP_USER").unwrap_or_default(),
            private_key_path: PathBuf::from(
                std::env::var("IRD_SFTP_KEY").unwrap_or_default(),
            ),
            remote_path: std::env::var("IRD_SFTP_REMOTE")
                .unwrap_or_else(|_| "/inbound/ird_batch.json".into()),
        };

        let interval_secs = std::env::var("IRD_INGEST_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);

        let run_once = std::env::var("IRD_RUN_ONCE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            transport,
            mock_fixture,
            sftp,
            interval_secs,
            run_once,
        })
    }
}
