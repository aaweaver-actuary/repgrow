use crate::config::load_default_config;

use super::toml_utils::load_config_type_from_file;
use anyhow::Result;
use derive_builder::Builder;
use serde::Deserialize;

/// Cache configuration.
/// - `entries`: Maximum number of entries in the cache (default: 200,000).
/// - `ttl_secs`: Time-to-live for cache entries in seconds (default: 3,600).
///
/// # Examples
/// ```
/// use repgrow::config::CacheConfig;
///
/// let cfg = CacheConfig::default();
/// assert_eq!(cfg.entries, 200000);
/// assert_eq!(cfg.ttl_secs, 3600);
///
/// let built_cfg = CacheConfig::builder()
///     .entries(500000)
///     .ttl_secs(3900)
///     .build()
///     .unwrap();
/// assert_eq!(built_cfg.entries, 500000);
/// assert_eq!(built_cfg.ttl_secs, 3900);
/// ```
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct CacheConfig {
    #[builder(default = "200000")]
    pub entries: u64,
    #[builder(default = "3600")]
    pub ttl_secs: u64,
}

impl CacheConfig {
    /// Load CacheConfig from a TOML file.
    /// # Arguments
    /// * `filename` - Path to the TOML configuration file.
    /// # Returns
    /// * `Result<CacheConfig>` - Loaded CacheConfig or an error.
    ///
    /// # Examples
    /// ```
    /// use repgrow::config::CacheConfig;
    /// let cfg_path = "src/config/default_config.toml";
    /// let cfg = CacheConfig::load(cfg_path).unwrap();
    /// assert_eq!(cfg.entries, 200000);
    /// assert_eq!(cfg.ttl_secs, 3600);
    /// ```
    pub fn load(filename: &str) -> Result<Self> {
        load_config_type_from_file(filename, "cache").and_then(|cfg| match cfg {
            super::toml_utils::ConfigTypes::Cache(c) => Ok(c),
            _ => Err(anyhow::anyhow!("Expected CacheConfig")),
        })
    }

    /// Create a builder for CacheConfig.
    /// # Returns
    /// * `CacheConfigBuilder` - A builder for CacheConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::CacheConfig;
    /// let cfg = CacheConfig::builder()
    ///     .entries(2000)
    ///     .ttl_secs(300)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(cfg.entries, 2000);
    /// assert_eq!(cfg.ttl_secs, 300);
    /// ```
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder::default()
    }
}

impl Default for CacheConfig {
    /// Load the default CacheConfig from the default configuration file.
    /// # Returns
    /// * `CacheConfig` - The default CacheConfig.
    /// # Panics
    /// Panics if the default configuration file cannot be loaded.
    /// # Examples
    /// ```
    /// use repgrow::config::CacheConfig;
    /// let cfg = CacheConfig::default();
    /// assert_eq!(cfg.entries, 200000);
    /// assert_eq!(cfg.ttl_secs, 3600);
    /// ```
    fn default() -> Self {
        load_default_config()
            .expect("Failed to load default config")
            .cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_toml() {
        let default_config = "src/config/default_config.toml".to_string();
        let cfg = CacheConfig::load(&default_config).unwrap();
        assert_eq!(cfg.entries, 200000);
        assert_eq!(cfg.ttl_secs, 3600);
    }

    #[test]
    fn test_default() {
        let cfg = CacheConfig::default();
        assert_eq!(cfg.entries, 200000);
        assert_eq!(cfg.ttl_secs, 3600);
    }

    #[test]
    fn test_builder() {
        let cfg = CacheConfigBuilder::default().build().unwrap();
        assert_eq!(cfg.entries, 200000);
        assert_eq!(cfg.ttl_secs, 3600);
    }

    #[test]
    fn test_builder_default_override() {
        let cfg = CacheConfigBuilder::default()
            .entries(100000)
            .ttl_secs(1800)
            .build()
            .unwrap();
        assert_eq!(cfg.entries, 100000);
        assert_eq!(cfg.ttl_secs, 1800);
    }

    #[test]
    fn test_debug_clone_deserialize() {
        let cfg1 = CacheConfigBuilder::default().build().unwrap();
        let cfg2 = cfg1.clone();
        assert_eq!(cfg1.entries, cfg2.entries);
        assert_eq!(cfg1.ttl_secs, cfg2.ttl_secs);

        let toml_str = r#"entries = 200000
ttl_secs = 3600"#;
        let cfg3: CacheConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg1.entries, cfg3.entries);
        assert_eq!(cfg1.ttl_secs, cfg3.ttl_secs);
    }
}
