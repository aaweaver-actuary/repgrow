use crate::config::{
    AppConfig, CacheConfig, HttpConfig, PolicyConfig, PopularityConfig, QualityConfig, RateConfig,
    SearchConfig,
};
use anyhow::Result;
use toml;

pub enum ConfigTypes {
    Cache(CacheConfig),
    Http(HttpConfig),
    Policy(PolicyConfig),
    Popularity(PopularityConfig),
    Quality(QualityConfig),
    Rate(RateConfig),
    Search(SearchConfig),
}

impl ConfigTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigTypes::Cache(_) => "cache",
            ConfigTypes::Http(_) => "http",
            ConfigTypes::Policy(_) => "policy",
            ConfigTypes::Popularity(_) => "popularity",
            ConfigTypes::Quality(_) => "quality",
            ConfigTypes::Rate(_) => "rate",
            ConfigTypes::Search(_) => "search",
        }
    }
}

pub fn load_toml_from_file(path: &str) -> Result<AppConfig> {
    let file_contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&file_contents)?)
}

pub fn load_default_config() -> Result<AppConfig> {
    let default_toml_file = "src/config/default_config.toml";
    load_toml_from_file(default_toml_file)
}

pub fn load_config_type_from_file(path: &str, config_type: &str) -> Result<ConfigTypes> {
    let file_contents = load_toml_from_file(path)?;

    match config_type {
        "cache" => Ok(ConfigTypes::Cache(file_contents.cache)),
        "http" => Ok(ConfigTypes::Http(file_contents.http)),
        "policy" => Ok(ConfigTypes::Policy(file_contents.policy)),
        "popularity" => Ok(ConfigTypes::Popularity(file_contents.popularity)),
        "quality" => Ok(ConfigTypes::Quality(file_contents.quality)),
        "rate" => Ok(ConfigTypes::Rate(file_contents.rate)),
        "search" => Ok(ConfigTypes::Search(file_contents.search)),
        _ => Err(anyhow::anyhow!("Unsupported config type")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_config_for_search() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.search.concurrency, 16);
    }

    #[test]
    fn test_load_default_config_for_policy() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.policy.my_side, Some("white".into()));
    }

    #[test]
    fn test_load_default_config_for_quality() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.quality.multi_pv, 4);
        assert_eq!(config.quality.source, "cloud");
    }

    #[test]
    fn test_load_default_config_for_popularity() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.popularity.speed, "all");
    }

    #[test]
    fn test_load_default_config_for_cache() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.cache.entries, 200000);
        assert_eq!(config.cache.ttl_secs, 3600);
    }

    #[test]
    fn test_load_default_config_for_http() {
        let config = load_default_config().expect("Failed to load default config");
        assert_eq!(config.http.rate_per_sec_cloud, 2);
        assert_eq!(config.http.rate_per_sec_explorer, 4);
        assert_eq!(config.http.retries, 3);
        assert_eq!(config.http.timeout_ms, 9000);
    }
}
