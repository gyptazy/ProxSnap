use anyhow::Result;
use reqwest::Client;
use crate::models::{ApiResponse, Node};

pub async fn list_nodes(client: &Client, base_url: &str) -> Result<Vec<Node>> {
    let url = format!("{}/api2/json/nodes", base_url);
    let resp: ApiResponse<Vec<Node>> =
        client.get(&url).send().await?.json().await?;

    Ok(resp.data)
}
