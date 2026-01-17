use clap::Parser;
use crate::helper::parse_date;
use chrono::NaiveDate;  

#[derive(Parser, Debug)]
#[command(
    name = "pve-snapshots",
    version,
    about = "List and inspect Proxmox snapshots"
)]
pub struct Cli {
    #[arg(long)]
    pub list: bool,

    #[arg(long, value_parser = parse_date)]
    pub delete_before: Option<NaiveDate>,
}