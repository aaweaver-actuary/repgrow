use super::toml_utils::{load_config_type_from_file, ConfigTypes};
use anyhow::{anyhow, Result};
use derive_builder::Builder;
use serde::Deserialize;

/// HTTP configuration.
/// - `timeout_ms`: Timeout for HTTP requests in milliseconds (default: 9,000).
/// - `retries`: Number of retries for failed requests (default: 3).
/// - `rate_per_sec_cloud`: Rate limit for requests to the cloud analysis service in requests per second (default: 2).
/// - `rate_per_sec_explorer`: Rate limit for requests to the explorer service in requests per second (default: 4).
#[derive(Debug, Clone, Deserialize, Builder)]
pub struct HttpConfig {
    /// Timeout for HTTP requests in milliseconds.
    #[builder(default = "9000")]
    pub timeout_ms: u64,
    /// Number of retries for failed requests.
    #[builder(default = "3")]
    pub retries: u32,
    /// Rate limit for requests to the cloud analysis service (requests per second).
    #[builder(default = "2")]
    pub rate_per_sec_cloud: u32,
    /// Rate limit for requests to the explorer service (requests per second).
    #[builder(default = "4")]
    pub rate_per_sec_explorer: u32,
}

impl HttpConfig {
    /// Load HttpConfig from a TOML file.
    /// # Arguments
    /// * `filename` - Path to the TOML configuration file.
    /// # Returns
    /// * `Result<HttpConfig>` - Loaded HttpConfig or an error.
    /// # Examples
    /// ```
    /// use repgrow::config::HttpConfig;
    /// let cfg_path = "src/config/default_config.toml";
    /// let cfg = HttpConfig::load(cfg_path).unwrap();
    /// assert_eq!(cfg.timeout_ms, 9000);
    /// assert_eq!(cfg.retries, 3);
    /// assert_eq!(cfg.rate_per_sec_cloud, 2);
    /// assert_eq!(cfg.rate_per_sec_explorer, 4);
    /// ```
    pub fn load(filename: &str) -> Result<Self> {
        load_config_type_from_file(filename, "http").and_then(|cfg| match cfg {
            ConfigTypes::Http(c) => Ok(c),
            _ => Err(anyhow!("Expected HttpConfig")),
        })
    }

    /// Create a builder for HttpConfig.
    /// # Returns
    /// * `HttpConfigBuilder` - A builder for HttpConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::HttpConfig;
    ///
    /// let cfg = HttpConfig::default();
    /// assert_eq!(cfg.timeout_ms, 9000);
    /// assert_eq!(cfg.retries, 3);
    /// assert_eq!(cfg.rate_per_sec_cloud, 2);
    /// assert_eq!(cfg.rate_per_sec_explorer, 4);
    ///
    /// let built_cfg = HttpConfig::builder()
    ///     .timeout_ms(5000)
    ///     .retries(5)
    ///     .rate_per_sec_cloud(3)
    ///     .rate_per_sec_explorer(6)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(built_cfg.timeout_ms, 5000);
    /// assert_eq!(built_cfg.retries, 5);
    /// assert_eq!(built_cfg.rate_per_sec_cloud, 3);
    /// assert_eq!(built_cfg.rate_per_sec_explorer, 6);
    /// ```
    pub fn builder() -> HttpConfigBuilder {
        HttpConfigBuilder::default()
    }
}

impl Default for HttpConfig {
    /// Load the default HttpConfig from the default configuration file.
    /// # Returns
    /// * `HttpConfig` - The default HttpConfig.
    /// # Panics
    /// Panics if the default configuration file cannot be loaded.
    /// # Examples
    /// ```
    /// use repgrow::config::HttpConfig;
    /// let cfg = HttpConfig::default();
    /// assert_eq!(cfg.timeout_ms, 9000);
    /// assert_eq!(cfg.retries, 3);
    /// assert_eq!(cfg.rate_per_sec_cloud, 2);
    /// assert_eq!(cfg.rate_per_sec_explorer, 4);
    /// ```
    fn default() -> Self {
        let filename = "src/config/default_config.toml";
        load_config_type_from_file(filename, "http")
            .and_then(|cfg| match cfg {
                ConfigTypes::Http(c) => Ok(c),
                _ => Err(anyhow!("Expected HttpConfig")),
            })
            .expect("Failed to load default HttpConfig")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_default() {
        let cfg = HttpConfig::default();
        assert_eq!(cfg.timeout_ms, 9000);
        assert_eq!(cfg.retries, 3);
        assert_eq!(cfg.rate_per_sec_cloud, 2);
        assert_eq!(cfg.rate_per_sec_explorer, 4);
    }

    #[test]
    fn test_http_config_load() {
        let filename = "src/config/default_config.toml";
        let cfg = HttpConfig::load(filename).unwrap();
        assert_eq!(cfg.timeout_ms, 9000);
        assert_eq!(cfg.retries, 3);
        assert_eq!(cfg.rate_per_sec_cloud, 2);
        assert_eq!(cfg.rate_per_sec_explorer, 4);
    }

    #[test]
    fn test_http_config_builder() {
        let cfg = HttpConfig::builder()
            .timeout_ms(5000)
            .retries(5)
            .rate_per_sec_cloud(3)
            .rate_per_sec_explorer(6)
            .build()
            .unwrap();
        assert_eq!(cfg.timeout_ms, 5000);
        assert_eq!(cfg.retries, 5);
        assert_eq!(cfg.rate_per_sec_cloud, 3);
        assert_eq!(cfg.rate_per_sec_explorer, 6);
    }

    #[test]
    fn test_http_config_builder_defaults() {
        let cfg = HttpConfig::builder().build().unwrap();
        let default_cfg = HttpConfig::default();

        assert_eq!(cfg.timeout_ms, default_cfg.timeout_ms);
        assert_eq!(cfg.retries, default_cfg.retries);
        assert_eq!(cfg.rate_per_sec_cloud, default_cfg.rate_per_sec_cloud);
        assert_eq!(cfg.rate_per_sec_explorer, default_cfg.rate_per_sec_explorer);
    }

    #[test]
    fn test_debug_clone_deserialize() {
        let toml_str = r#"timeout_ms = 9000
retries = 3
rate_per_sec_cloud = 2
rate_per_sec_explorer = 4"#;
        let cfg1: HttpConfig = toml::from_str(toml_str).unwrap();
        let cfg2 = cfg1.clone();
        assert_eq!(cfg1.timeout_ms, cfg2.timeout_ms);
        assert_eq!(cfg1.retries, cfg2.retries);
        assert_eq!(cfg1.rate_per_sec_cloud, cfg2.rate_per_sec_cloud);
        assert_eq!(cfg1.rate_per_sec_explorer, cfg2.rate_per_sec_explorer);
    }

    #[test]
    fn test_http_config_panic_on_invalid_file() {
        let filename = "non_existent_file.toml";
        let result = HttpConfig::load(filename);
        assert!(result.is_err());
    }
}
