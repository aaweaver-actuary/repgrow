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

#[cfg(test)]
mod tests {
    use crate::domain::Centipawns;

    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn sample_toml() -> String {
        let default_config = "src/config/default_config.toml".to_string();
        fs::read_to_string(default_config).expect("Failed to read default config")
    }

    #[test]
    fn test_load_from_toml() {
        let toml_str = sample_toml();
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", toml_str).unwrap();
        let path = file.path().to_str().unwrap();
        let cfg = AppConfig::load(path).unwrap();
        assert_eq!(cfg.search.concurrency, 16);
        assert_eq!(cfg.policy.cp_window, Centipawns::from_int(50));
        assert_eq!(cfg.quality.source, "cloud");
        assert_eq!(cfg.popularity.source, "explorer");
        assert_eq!(cfg.http.timeout_ms, 9000);
        assert_eq!(cfg.cache.entries, 200000);
        assert_eq!(cfg.rate.cloud_per_sec, 2);
    }

    #[test]
    fn test_debug_clone_deserialize() {
        let toml_str = sample_toml();
        let cfg: AppConfig = toml::from_str(&toml_str).unwrap();
        let cfg2 = cfg.clone();
        assert_eq!(cfg2.search.concurrency, cfg.search.concurrency);
        let dbg = format!("{:?}", cfg2);
        assert!(dbg.contains("AppConfig"));
    }
}
