use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub timeout_ms: u64,
    pub retries: u32,
    pub rate_per_sec_cloud: u32,
    pub rate_per_sec_explorer: u32,
}
