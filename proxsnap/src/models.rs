use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub node: String,
}

#[derive(Debug, Deserialize)]
pub struct Guest {
    pub vmid: u64,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub name: String,
}

#[derive(Debug)]
pub enum GuestKind {
    Qemu,
    Lxc,
}

#[derive(Debug)]
pub struct GuestSnapshots {
    pub kind: GuestKind,
    pub vmid: u64,
    pub name: Option<String>,
    pub snapshots: Vec<Snapshot>,
}

pub type Inventory = HashMap<String, Vec<GuestSnapshots>>;
