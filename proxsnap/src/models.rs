use serde::Deserialize;

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
