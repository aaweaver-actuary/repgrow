use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub entries: u64,
    pub ttl_secs: u64,
}

impl CacheConfig {
    pub fn load(filename: &str) -> Result<Self> {
        let file_content = read_to_string(filename)?;
        Ok(toml::from_str(&file_content)?)
    }
}
