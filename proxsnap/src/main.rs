mod config;
mod client;
mod models;
mod api;
mod helper;
mod cli;

use anyhow::Result;
use config::ProxmoxConfig;
use api::{nodes, qemu, lxc};
use std::collections::HashMap;
use models::{Inventory, GuestSnapshots, GuestKind};
use helper::report_inventory;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // let cfg = ProxmoxConfig::from_env();
    let cfg = ProxmoxConfig {
        base_url: "https://10.10.10.11:8006".into(),
        api_token: "PVEAPIToken=root@pam!test=ac1079bb-366f-46ab-9658-8a3262da6077".into(),
        insecure_tls: true,
    };
    let client = client::create_client(&cfg)?;
    let nodes = nodes::list_nodes(&client, &cfg.base_url).await?;
    let mut inventory: Inventory = HashMap::new();

    for node in nodes {
        let mut guests = Vec::new();

        for vm in qemu::list_vms(&client, &cfg.base_url, &node.node).await? {
            let snaps = qemu::list_snapshots(
                &client,
                &cfg.base_url,
                &node.node,
                vm.vmid,
            ).await?;

            guests.push(GuestSnapshots {
                kind: GuestKind::Qemu,
                vmid: vm.vmid,
                name: vm.name,
                snapshots: snaps,
            });
        }

        for ct in lxc::list_containers(&client, &cfg.base_url, &node.node).await? {
            let snaps = lxc::list_snapshots(
                &client,
                &cfg.base_url,
                &node.node,
                ct.vmid,
            ).await?;

            guests.push(GuestSnapshots {
                kind: GuestKind::Lxc,
                vmid: ct.vmid,
                name: ct.name,
                snapshots: snaps,
            });
        }

        inventory.insert(node.node.clone(), guests);
    }

    if cli.list {
    report_inventory(&inventory);
    }

    if let Some(date) = cli.delete_before {
        println!("Deleting snapshots before: {}", date);
    }

    Ok(())
}
