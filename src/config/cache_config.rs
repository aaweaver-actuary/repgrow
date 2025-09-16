use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub entries: u64,
    pub ttl_secs: u64,
}
