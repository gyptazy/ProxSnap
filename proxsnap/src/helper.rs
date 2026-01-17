use crate::models::{Inventory, GuestKind};

pub fn report_inventory(inv: &Inventory) {
    for (node, guests) in inv {
        println!("Node: {}", node);

        for g in guests {
            let real = g.snapshots.iter()
                .filter(|s| s.name != "current")
                .count();

            let name = g.name.as_deref().unwrap_or("<unnamed>");
            let kind = match g.kind {
                GuestKind::Qemu => "VM",
                GuestKind::Lxc => "CT",
            };

            if real > 0 {
                println!("  ⚠ {} {} ({}) has {} snapshots", kind, g.vmid, name, real);
            } else {
                println!("  ✓ {} {} ({}) has no snapshots", kind, g.vmid, name);
            }
        }
    }
}
