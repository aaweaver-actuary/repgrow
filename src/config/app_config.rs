use std::fs;

use serde::Deserialize;

use crate::config::{
    CacheConfig, HttpConfig, PolicyConfig, PopularityConfig, QualityConfig, RateConfig,
    SearchConfig,
};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub search: SearchConfig,
    pub policy: PolicyConfig,
    pub quality: QualityConfig,
    pub popularity: PopularityConfig,
    pub http: HttpConfig,
    pub cache: CacheConfig,
    pub rate: RateConfig,
}

impl AppConfig {
    /// Load config from a TOML file.
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path)?;
        Ok(toml::from_str(&s)?)
    }
}
