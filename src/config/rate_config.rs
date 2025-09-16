use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RateConfig {
    pub cloud_per_sec: u32,
    pub explorer_per_sec: u32,
}
