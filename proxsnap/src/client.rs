use reqwest::{Client, header};
use anyhow::Result;
use crate::config::ProxmoxConfig;

pub fn create_client(cfg: &ProxmoxConfig) -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        cfg.api_token.parse()?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(cfg.insecure_tls)
        .build()?;

    Ok(client)
}
