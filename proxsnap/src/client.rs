use std::time::Duration;
use reqwest::{Client, header};
use anyhow::Result;
use crate::config::ProxmoxConfig;
use crate::config_file::FileConfig;

pub fn create_client(cfg: &ProxmoxConfig) -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        cfg.api_token.parse()?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(cfg.insecure_tls)
        .build()?;

    Ok(client)
}

impl ProxmoxConfig {
    pub fn from_file(cfg: FileConfig) -> Self {
        let api = cfg.proxmox_api;

        Self {
            base_url: format!("https://{}:8006", api.hosts),
            api_token: format!(
                "PVEAPIToken={}!{}={}",
                api.user,
                api.token_id,
                api.token_secret
            ),
            insecure_tls: !api.ssl_verification,
        }
    }
}