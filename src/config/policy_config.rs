use super::toml_utils::{load_config_type_from_file, ConfigTypes};
use crate::domain::{Centipawns, PlayRate};
use anyhow::{anyhow, Result};
use derive_builder::Builder;
use serde::Deserialize;
use shakmaty::Color;

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct PolicyConfig {
    /// Side to base the repertoire around. The "best" moves will be chosen for this side.
    pub my_side: Option<String>,
    pub cp_window: Centipawns,
    pub min_play_rate: PlayRate,
}

impl PolicyConfig {
    /// Load PolicyConfig from a TOML file.
    /// # Arguments
    /// * `filename` - Path to the TOML configuration file.
    /// # Returns
    /// * `Result<PolicyConfig>` - Loaded PolicyConfig or an error.
    /// # Examples
    /// ```
    /// use repgrow::config::PolicyConfig;
    /// use repgrow::domain::{Centipawns, PlayRate};
    ///
    /// let cfg_path = "src/config/default_config.toml";
    /// let cfg = PolicyConfig::load(cfg_path).unwrap();
    /// assert_eq!(cfg.my_side, Some("white".to_string()));
    /// assert_eq!(cfg.cp_window, Centipawns::from_int(50));
    /// assert_eq!(cfg.min_play_rate, PlayRate::new(0.07));
    /// ```
    pub fn load(filename: &str) -> Result<Self> {
        load_config_type_from_file(filename, "policy").and_then(|cfg| match cfg {
            ConfigTypes::Policy(c) => Ok(c),
            _ => Err(anyhow!("Expected PolicyConfig")),
        })
    }

    /// Create a builder for PolicyConfig.
    /// # Returns
    /// * `PolicyConfigBuilder` - A builder for PolicyConfig.
    /// # Examples
    /// ```
    /// use repgrow::config::PolicyConfig;
    /// use repgrow::domain::{Centipawns, PlayRate};
    ///
    /// let cfg = PolicyConfig::builder()
    ///     .my_side(Some("black".to_string()))
    ///     .cp_window(Centipawns::from_int(100))
    ///     .min_play_rate(PlayRate::new(0.02))
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(cfg.my_side, Some("black".to_string()));
    /// assert_eq!(cfg.cp_window, Centipawns::from_int(100));
    /// assert_eq!(cfg.min_play_rate, PlayRate::new(0.02));
    /// ```
    pub fn builder() -> PolicyConfigBuilder {
        PolicyConfigBuilder::default()
    }

    pub fn resolve_side_override(&self, cli_side: &str) -> Result<Color> {
        let s = if !cli_side.is_empty() {
            Some(cli_side.to_string())
        } else {
            self.my_side.clone()
        };
        match s.as_deref() {
            Some("white") => Ok(Color::White),
            Some("black") => Ok(Color::Black),
            _ => anyhow::bail!("side must be white|black"),
        }
    }
}

impl Default for PolicyConfig {
    /// Load the default PolicyConfig from the default configuration file.
    /// # Returns
    /// * `PolicyConfig` - The default PolicyConfig.
    /// # Panics
    /// Panics if the default configuration file cannot be loaded.
    /// # Examples
    /// ```
    /// use repgrow::config::PolicyConfig;
    /// use repgrow::domain::{Centipawns, PlayRate};
    ///
    /// let cfg = PolicyConfig::default();
    /// assert_eq!(cfg.my_side, Some("white".to_string()));
    /// assert_eq!(cfg.cp_window, Centipawns::from_int(50));
    /// assert_eq!(cfg.min_play_rate, PlayRate::new(0.07));
    /// ```
    fn default() -> Self {
        let default_toml_file = "src/config/default_config.toml";
        PolicyConfig::load(default_toml_file).expect("Failed to load default PolicyConfig")
    }
}
