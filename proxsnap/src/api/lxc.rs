use anyhow::Result;
use reqwest::Client;
use crate::models::{ApiResponse, Guest, Snapshot};

pub async fn list_containers(
    client: &Client,
    base_url: &str,
    node: &str,
) -> Result<Vec<Guest>> {
    let url = format!("{}/api2/json/nodes/{}/lxc", base_url, node);
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
        "{}/api2/json/nodes/{}/lxc/{}/snapshot",
        base_url, node, vmid
    );
    let resp: ApiResponse<Vec<Snapshot>> =
        client.get(&url).send().await?.json().await?;

    Ok(resp.data)
}

pub async fn delete_snapshot(
    client: &Client,
    base_url: &str,
    node: &str,
    vmid: u64,
    snapshot: &str,
) -> Result<()> {
    let url = format!(
        "{}/api2/json/nodes/{}/lxc/{}/snapshot/{}",
        base_url, node, vmid, snapshot
    );

    client
        .delete(&url)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}