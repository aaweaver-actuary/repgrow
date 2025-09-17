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
    /// Get the string representation of the config type.
    /// # Returns
    /// * `&'static str` - The string representation of the config type.
    /// # Examples
    /// ```
    /// use repgrow::config::toml_utils::{ConfigTypes, load_config_type_from_file};
    /// let cfg_path = "src/config/default_config.toml";
    /// let http_cfg = load_config_type_from_file(cfg_path, "http").unwrap();
    /// assert_eq!(http_cfg.as_str(), "http");
    ///
    /// let cache_cfg = load_config_type_from_file(cfg_path, "cache").unwrap();
    /// assert_eq!(cache_cfg.as_str(), "cache");
    ///
    /// let policy_cfg = load_config_type_from_file(cfg_path, "policy").unwrap();
    /// assert_eq!(policy_cfg.as_str(), "policy");
    ///
    /// let popularity_cfg = load_config_type_from_file(cfg_path, "popularity").unwrap();
    /// assert_eq!(popularity_cfg.as_str(), "popularity");
    ///
    /// let quality_cfg = load_config_type_from_file(cfg_path, "quality").unwrap();
    /// assert_eq!(quality_cfg.as_str(), "quality");
    ///
    /// let rate_cfg = load_config_type_from_file(cfg_path, "rate").unwrap();
    /// assert_eq!(rate_cfg.as_str(), "rate");
    ///
    /// let search_cfg = load_config_type_from_file(cfg_path, "search").unwrap();
    /// assert_eq!(search_cfg.as_str(), "search");
    /// ```
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

/// Load the entire TOML configuration from a file and into an AppConfig struct.
/// # Arguments
/// * `path` - Path to the TOML configuration file.
/// # Returns
/// * `Result<AppConfig>` - Loaded AppConfig or an error.
/// # Examples
/// ```
/// use repgrow::config::toml_utils::load_toml_from_file;
/// let cfg_path = "src/config/default_config.toml";
/// let cfg = load_toml_from_file(cfg_path).unwrap();
/// assert_eq!(cfg.http.timeout_ms, 9000);
/// assert_eq!(cfg.http.retries, 3);
/// assert_eq!(cfg.http.rate_per_sec_cloud, 2);
/// ```
pub fn load_toml_from_file(path: &str) -> Result<AppConfig> {
    let file_contents = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&file_contents)?)
}

/// Load the default AppConfig from the default configuration file.
/// # Returns
/// * `Result<AppConfig>` - The default AppConfig or an error.
/// # Examples
/// ```
/// use repgrow::config::toml_utils::load_default_config;
/// let cfg = load_default_config().unwrap();
/// assert_eq!(cfg.http.timeout_ms, 9000);
/// assert_eq!(cfg.http.retries, 3);
/// assert_eq!(cfg.http.rate_per_sec_cloud, 2);
/// ```
pub fn load_default_config() -> Result<AppConfig> {
    let default_toml_file = "src/config/default_config.toml";
    load_toml_from_file(default_toml_file)
}

/// Load a specific configuration type from a TOML file.
/// # Arguments
/// * `path` - Path to the TOML configuration file.
/// * `config_type` - The type of configuration to load (e.g., "cache", "http", "policy", etc.).
/// # Returns
/// * `Result<ConfigTypes>` - Loaded configuration of the specified type or an error.
/// # Examples
/// ```
/// use repgrow::config::toml_utils::{ConfigTypes, load_config_type_from_file};
/// let cfg_path = "src/config/default_config.toml";
/// let http_cfg = load_config_type_from_file(cfg_path, "http").unwrap();
/// assert_eq!(http_cfg.as_str(), "http");
///
/// let cache_cfg = load_config_type_from_file(cfg_path, "cache").unwrap();
/// let policy_cfg = load_config_type_from_file(cfg_path, "policy").unwrap();
/// let popularity_cfg = load_config_type_from_file(cfg_path, "popularity").unwrap();
/// let quality_cfg = load_config_type_from_file(cfg_path, "quality").unwrap();
/// let rate_cfg = load_config_type_from_file(cfg_path, "rate").unwrap();
/// let search_cfg = load_config_type_from_file(cfg_path, "search").unwrap();
/// let http_cfg = load_config_type_from_file(cfg_path, "http").unwrap();
///
/// assert_eq!(http_cfg.as_str(), "http");
/// assert_eq!(cache_cfg.as_str(), "cache");
/// assert_eq!(policy_cfg.as_str(), "policy");
/// assert_eq!(popularity_cfg.as_str(), "popularity");
/// assert_eq!(quality_cfg.as_str(), "quality");
/// assert_eq!(rate_cfg.as_str(), "rate");
/// assert_eq!(search_cfg.as_str(), "search");
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
