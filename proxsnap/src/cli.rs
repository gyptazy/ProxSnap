use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "pve-snapshots",
    version,
    about = "List and inspect Proxmox snapshots"
)]
pub struct Cli {
    #[arg(long)]
    pub list: bool,
}