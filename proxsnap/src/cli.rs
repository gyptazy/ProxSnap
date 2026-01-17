use clap::Parser;
use crate::helper::parse_date;
use chrono::NaiveDate;  

#[derive(Parser, Debug)]
#[command(
    name = "ProxSnap",
    version,
    about = "Snapshot management tool for Proxmox clusters.\nAuthor: Florian Paul Azim Hoberg @gyptazy <contact@gyptazy.com>"
)]
pub struct Cli {
    #[arg(short = 'l', long, help = "List snapshots for all VMs & CTs")]
    pub list: bool,

    #[arg(short = 'd', long, help = "Delete all snapshots before a given date (YYYY-MM-DD)", value_parser = parse_date)]
    pub delete_before: Option<NaiveDate>,
}