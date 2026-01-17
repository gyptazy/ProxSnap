pub struct ProxmoxConfig {
    pub base_url: String,
    pub api_token: String,
    pub insecure_tls: bool,
}

impl ProxmoxConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("PVE_URL")
                .expect("PVE_URL not set"),
            api_token: std::env::var("PVE_API_TOKEN")
                .expect("PVE_API_TOKEN not set"),
            insecure_tls: true,
        }
    }
}
