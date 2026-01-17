use anyhow::Result;
use reqwest::Client;
use crate::models::{ApiResponse, Guest, Snapshot};

pub async fn list_vms(
    client: &Client,
    base_url: &str,
    node: &str,
) -> Result<Vec<Guest>> {
    let url = format!("{}/api2/json/nodes/{}/qemu", base_url, node);
    let resp: ApiResponse<Vec<Guest>> =
        client.get(&url).send().await?.json().await?;

    Ok(resp.data)
}

pub async fn list_snapshots(
    client: &Client,
    base_url: &str,
    node: &str,
    vmid: u64,
) -> Result<Vec<Snapshot>> {
    let url = format!(
        "{}/api2/json/nodes/{}/qemu/{}/snapshot",
        base_url, node, vmid
    );
    let resp: ApiResponse<Vec<Snapshot>> =
        client.get(&url).send().await?.json().await?;

    Ok(resp.data)
}
