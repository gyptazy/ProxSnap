mod config;
mod client;
mod models;
mod api;

use anyhow::Result;
use config::ProxmoxConfig;
use api::{nodes, qemu, lxc};

#[tokio::main]
async fn main() -> Result<()> {
    // let cfg = ProxmoxConfig::from_env();
    let cfg = ProxmoxConfig {
        base_url: "https://10.10.10.11:8006".into(),
        api_token: "PVEAPIToken=root@pam!test=ac1079bb-366f-46ab-9658-8a3262da6077".into(),
        insecure_tls: true,
    };
    let client = client::create_client(&cfg)?;

    let nodes = nodes::list_nodes(&client, &cfg.base_url).await?;

    for node in nodes {
        println!("Node: {}", node.node);

        for vm in qemu::list_vms(&client, &cfg.base_url, &node.node).await? {
            let snaps = qemu::list_snapshots(
                &client,
                &cfg.base_url,
                &node.node,
                vm.vmid,
            )
            .await?;

            report("VM", vm.vmid, &vm.name, snaps.len());
        }

        for ct in lxc::list_containers(&client, &cfg.base_url, &node.node).await? {
            let snaps = lxc::list_snapshots(
                &client,
                &cfg.base_url,
                &node.node,
                ct.vmid,
            )
            .await?;

            report("CT", ct.vmid, &ct.name, snaps.len());
        }
    }

    Ok(())
}

fn report(kind: &str, vmid: u64, name: &Option<String>, count: usize) {
    let name = name.as_deref().unwrap_or("<unnamed>");
    let real = count.saturating_sub(1);

    if real > 0 {
        println!("  ⚠ {} {} ({}) has {} snapshots", kind, vmid, name, real);
    } else {
        println!("  ✓ {} {} ({}) has no snapshots", kind, vmid, name);
    }
}
