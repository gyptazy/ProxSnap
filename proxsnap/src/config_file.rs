use serde::Deserialize;
use std::fs;
use anyhow::{Context, Result};

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub proxmox_api: ProxmoxApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProxmoxApiConfig {
    pub hosts: String,
    pub user: String,
    pub token_id: String,
    pub token_secret: String,
    pub ssl_verification: bool,
    pub timeout: u64,
}

impl FileConfig {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path))?;

        let cfg: FileConfig = serde_yaml::from_str(&content)
            .with_context(|| format!("failed to parse {}", path))?;

        Ok(cfg)
    }
}