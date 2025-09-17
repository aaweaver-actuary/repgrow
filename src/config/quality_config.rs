use derive_builder::Builder;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct QualityConfig {
    #[builder(default = "\"cloud\".to_string()")]
    pub source: String,
    #[builder(default = "5")]
    pub multi_pv: usize,
    #[builder(default = "\"https://lichess.org/api/cloud-eval\".to_string()")]
    pub base_url: String,
}

impl QualityConfig {
    /// Load QualityConfig from a TOML file.
    /// # Arguments
    /// * `filename` - Path to the TOML configuration file.
    /// # Returns
    /// * `Result<QualityConfig>` - Loaded QualityConfig or an error.
    /// # Examples
    /// ```
    /// use repgrow::config::QualityConfig;
    /// let cfg_path = "src/config/default_config.toml";
    /// let cfg = QualityConfig::load(cfg_path).unwrap();
    /// assert_eq!(cfg.source, "cloud".to_string());
    /// assert_eq!(cfg.multi_pv, 4);
    /// assert_eq!(cfg.base_url, "https://lichess.org/api/cloud-eval".to_string());
    /// ```
    pub fn load(filename: &str) -> anyhow::Result<Self> {
        crate::config::toml_utils::load_config_type_from_file(filename, "quality").and_then(|cfg| {
            match cfg {
                crate::config::toml_utils::ConfigTypes::Quality(c) => Ok(c),
                _ => Err(anyhow::anyhow!("Expected QualityConfig")),
            }
        })
    }

    /// Create a builder for QualityConfig.
    /// # Returns
    /// * `QualityConfigBuilder` - A builder for QualityConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::QualityConfig;
    ///
    /// let cfg = QualityConfig::builder()
    ///     .source("andy".to_string())
    ///     .multi_pv(25)
    ///     .base_url("https://lichess.org/api/cloud-eval-andy".to_string())
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(cfg.source, "andy".to_string());
    /// assert_eq!(cfg.multi_pv, 25);
    /// assert_eq!(cfg.base_url, "https://lichess.org/api/cloud-eval-andy".to_string());
    /// ```
    pub fn builder() -> QualityConfigBuilder {
        QualityConfigBuilder::default()
    }
}

impl Default for QualityConfig {
    /// Load the default QualityConfig from the default configuration file.
    /// # Returns
    /// * `QualityConfig` - The default QualityConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::QualityConfig;
    /// let cfg = QualityConfig::default();
    /// assert_eq!(cfg.source, "cloud".to_string());
    /// assert_eq!(cfg.multi_pv, 4);
    /// assert_eq!(cfg.base_url, "https://lichess.org/api/cloud-eval".to_string());
    /// ```
    fn default() -> Self {
        let cfg_path = "src/config/default_config.toml";
        QualityConfig::load(cfg_path).unwrap() // panic if cannot load default config --- IGN
    }
}
