mod config;
mod client;
mod models;
mod api;
mod helper;
mod cli;
mod config_file;

use anyhow::Result;
use config::ProxmoxConfig;
use api::{nodes, qemu, lxc};
use std::collections::HashMap;
use models::{Inventory, GuestSnapshots, GuestKind};
use helper::report_inventory;
use clap::Parser;
use cli::Cli;
use config_file::FileConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.as_deref().unwrap_or("/etc/proxsnap/proxsnap.yaml");
    let file_cfg = FileConfig::load(config_path)?;
    let cfg = ProxmoxConfig::from_file(file_cfg);
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

    if let Some(cutoff) = cli.date {
        helper::report_snapshots_older_than(&inventory, cutoff, cli.remove, &client, &cfg.base_url).await?;
        return Ok(());
    }


    if cli.list {
    report_inventory(&inventory);
    return Ok(());
    }

    Ok(())
}
