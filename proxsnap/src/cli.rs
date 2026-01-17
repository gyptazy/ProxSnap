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

    #[arg(short = 'c', long, help = "Path to ProxSnap config file (default: /etc/proxsnap/proxsnap.yaml)", value_name = "FILE")]
    pub config: Option<String>,

    #[arg(short = 'l', long, help = "List snapshots for all VMs & CTs")]
    pub list: bool,

    #[arg(short = 'd', long, help = "List snapshots for all VMs & CTs before a specific date (YYYY-MM-DD)", value_parser = parse_date)]
    pub date: Option<NaiveDate>,

    #[arg(short = 'r', long, help = "Remove snapshots after before a given date")]
    pub remove: bool,

}