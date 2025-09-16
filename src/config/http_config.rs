use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub timeout_ms: u64,
    pub retries: u32,
}
