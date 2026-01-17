use crate::models::{Inventory, GuestKind};
use chrono::{DateTime, Local, TimeZone};
use chrono_humanize::HumanTime;

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
                // let when = human_time(snap.snaptime);
                let when = format_snapshot_time(snap.snaptime);
                println!("      • {} ({})", snap.name, when);
            }
        }
    }
}

// pub fn report_inventory(inv: &Inventory) {
//     for (node, guests) in inv {
//         println!("Node: {}", node);

//         for g in guests {
//             let real = g.snapshots.iter()
//                 .filter(|s| s.name != "current")
//                 .count();

//             let name = g.name.as_deref().unwrap_or("<unnamed>");
//             let kind = match g.kind {
//                 GuestKind::Qemu => "VM",
//                 GuestKind::Lxc => "CT",
//             };

//             if real > 0 {
//                 println!("  ⚠ {} {} ({}) has {} snapshots", kind, g.vmid, name, real);
//             } else {
//                 println!("  ✓ {} {} ({}) has no snapshots", kind, g.vmid, name);
//             }
//         }
//     }
// }

// fn human_time(ts: Option<i64>) -> String {
//     match ts {
//         Some(ts) => {
//             let dt: DateTime<Local> = Local.timestamp_opt(ts, 0).unwrap();
//             HumanTime::from(dt).to_string()
//         }
//         None => "n/a".into(),
//     }
// }

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