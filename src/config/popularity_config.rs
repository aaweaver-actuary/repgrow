use derive_builder::Builder;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct PopularityConfig {
    #[builder(default = "\"explorer\".to_string()")]
    pub source: String, // "explorer" for now
    #[builder(default = "\"https://explorer.lichess.ovh\".to_string()")]
    pub base_url: String,
    #[builder(default = "\"all\".to_string()")]
    pub speed: String,
    #[builder(default = "800")]
    pub min_rating: u32,
    #[builder(default = "2000")]
    pub max_rating: u32,
    #[builder(default = "2019")]
    pub since_year: u32,
}

impl PopularityConfig {
    /// Load PopularityConfig from a TOML file.
    /// # Arguments
    /// * `filename` - Path to the TOML configuration file.
    /// # Returns
    /// * `Result<PopularityConfig>` - Loaded PopularityConfig or an error.
    /// # Examples
    /// ```
    /// use repgrow::config::PopularityConfig;
    /// let cfg_path = "src/config/default_config.toml";
    /// let cfg = PopularityConfig::load(cfg_path).unwrap();
    /// assert_eq!(cfg.source, "explorer".to_string());
    /// assert_eq!(cfg.base_url, "https://explorer.lichess.ovh/lichess".to_string());
    /// assert_eq!(cfg.speed, "all".to_string());
    /// assert_eq!(cfg.min_rating, 800);
    /// assert_eq!(cfg.max_rating, 2000);
    /// assert_eq!(cfg.since_year, 2019);
    /// ```
    pub fn load(filename: &str) -> anyhow::Result<Self> {
        crate::config::toml_utils::load_config_type_from_file(filename, "popularity").and_then(
            |cfg| match cfg {
                crate::config::toml_utils::ConfigTypes::Popularity(c) => Ok(c),
                _ => Err(anyhow::anyhow!("Expected PopularityConfig")),
            },
        )
    }

    /// Create a builder for PopularityConfig.
    /// # Returns
    /// * `PopularityConfigBuilder` - A builder for PopularityConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::PopularityConfig;
    ///
    /// let built_cfg = PopularityConfig::builder()
    ///     .source("explorer".to_string())
    ///     .base_url("https://explorer.lichess.ovh".to_string())
    ///     .speed("blitz".to_string())
    ///     .min_rating(1000)
    ///     .max_rating(2200)
    ///     .since_year(2020)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(built_cfg.source, "explorer".to_string());
    /// assert_eq!(built_cfg.base_url, "https://explorer.lichess.ovh".to_string());
    /// assert_eq!(built_cfg.speed, "blitz".to_string());
    /// assert_eq!(built_cfg.min_rating, 1000);
    /// assert_eq!(built_cfg.max_rating, 2200);
    /// assert_eq!(built_cfg.since_year, 2020);
    /// ```
    pub fn builder() -> PopularityConfigBuilder {
        PopularityConfigBuilder::default()
    }
}

impl Default for PopularityConfig {
    /// Load the default PopularityConfig from the default configuration file.
    /// # Returns
    /// * `PopularityConfig` - The default PopularityConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::PopularityConfig;
    ///
    /// let cfg = PopularityConfig::default();
    /// assert_eq!(cfg.source, "explorer".to_string());
    /// assert_eq!(cfg.base_url, "https://explorer.lichess.ovh/lichess".to_string());
    /// assert_eq!(cfg.speed, "all".to_string());
    /// assert_eq!(cfg.min_rating, 800);
    /// assert_eq!(cfg.max_rating, 2000);
    /// assert_eq!(cfg.since_year, 2019);
    /// ```
    fn default() -> Self {
        let filename = "src/config/default_config.toml";
        Self::load(filename).expect("Failed to load default config")
    }
}
