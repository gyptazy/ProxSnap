use crate::models::{Inventory, GuestKind};
use chrono::{DateTime, Local, TimeZone, NaiveDate, NaiveDateTime};
use chrono_humanize::HumanTime;
use crate::api::{qemu, lxc};

pub fn report_inventory(inv: &Inventory) {
    for (node, guests) in inv {
        println!("Node: {}", node);

        for g in guests {
            let name = g.name.as_deref().unwrap_or("<unnamed>");
            let kind = match g.kind {
                GuestKind::Qemu => "VM",
                GuestKind::Lxc => "CT",
            };

            let real_snaps: Vec<_> = g.snapshots
                .iter()
                .filter(|s| s.name != "current")
                .collect();

            if real_snaps.is_empty() {
                println!("  ✓ {} {} ({}) has no snapshots", kind, g.vmid, name);
                continue;
            }

            println!(
                "  ⚠ {} {} ({}) has {} snapshots:",
                kind,
                g.vmid,
                name,
                real_snaps.len()
            );

            for snap in real_snaps {
                let when = format_snapshot_time(snap.snaptime);
                println!("      • {} ({})", snap.name, when);
            }
        }
    }
}

fn format_snapshot_time(ts: Option<i64>) -> String {
    match ts {
        Some(ts) => {
            let dt: DateTime<Local> = Local.timestamp_opt(ts, 0).unwrap();

            let absolute = dt.format("%Y-%m-%d %H:%M:%S");
            let relative = HumanTime::from(dt).to_string();

            format!("{} ({})", absolute, relative)
        }
        None => "n/a".into(),
    }
}

pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| "expected format: YYYY-MM-DD".to_string())
}

// pub fn report_snapshots_older_than(
//     inv: &Inventory,
//     cutoff: NaiveDate,
// ) {
//     println!(
//         "Checking for snapshots older than {}",
//         cutoff.format("%Y-%m-%d")
//     );

//     for (node, guests) in inv {
//         for g in guests {
//             let kind = match g.kind {
//                 GuestKind::Qemu => "VM",
//                 GuestKind::Lxc => "CT",
//             };

//             for snap in &g.snapshots {
//                 if snap.name == "current" {
//                     continue;
//                 }

//                 let ts = match snap.snaptime {
//                     Some(ts) => ts,
//                     None => continue,
//                 };

//                 let snap_date = match NaiveDateTime::from_timestamp_opt(ts, 0) {
//                     Some(dt) => dt.date(),
//                     None => continue,
//                 };

//                 if snap_date < cutoff {
//                     let name = g.name.as_deref().unwrap_or("<unnamed>");

//                     println!(
//                         "⚠ {} {} ({}) snapshot '{}' is older than cutoff ({} < {})",
//                         kind,
//                         g.vmid,
//                         name,
//                         snap.name,
//                         snap_date.format("%Y-%m-%d"),
//                         cutoff.format("%Y-%m-%d")
//                     );
//                 }
//             }
//         }
//     }
// }

pub async fn report_snapshots_older_than(
    inv: &Inventory,
    cutoff: NaiveDate,
    remove: bool,
    client: &reqwest::Client,
    base_url: &str,
) -> anyhow::Result<()> {
    println!(
        "Checking for snapshots older than {}{}",
        cutoff.format("%Y-%m-%d"),
        if remove { " (REMOVING)" } else { " (dry-run)" }
    );

    for (node, guests) in inv {
        for g in guests {
            let kind_str = match g.kind {
                GuestKind::Qemu => "VM",
                GuestKind::Lxc => "CT",
            };

            for snap in &g.snapshots {
                if snap.name == "current" {
                    continue;
                }

                let ts = match snap.snaptime {
                    Some(ts) => ts,
                    None => continue,
                };

                let snap_date = match NaiveDateTime::from_timestamp_opt(ts, 0) {
                    Some(dt) => dt.date(),
                    None => continue,
                };

                if snap_date < cutoff {
                    let name = g.name.as_deref().unwrap_or("<unnamed>");

                    println!(
                        "⚠ {} {} ({}) snapshot '{}' is older than cutoff ({} < {})",
                        kind_str,
                        g.vmid,
                        name,
                        snap.name,
                        snap_date.format("%Y-%m-%d"),
                        cutoff.format("%Y-%m-%d")
                    );

                    if remove {
                        match g.kind {
                            GuestKind::Qemu => {
                                qemu::delete_snapshot(
                                    client,
                                    base_url,
                                    node,
                                    g.vmid,
                                    &snap.name,
                                ).await?;
                            }
                            GuestKind::Lxc => {
                                lxc::delete_snapshot(
                                    client,
                                    base_url,
                                    node,
                                    g.vmid,
                                    &snap.name,
                                ).await?;
                            }
                        }

                        println!(
                            "  → deleted snapshot '{}' of {} {}",
                            snap.name,
                            kind_str,
                            g.vmid
                        );
                    }
                }
            }
        }
    }

    Ok(())
}