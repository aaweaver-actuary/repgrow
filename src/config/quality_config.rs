use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct QualityConfig {
    pub provider: String, // "cloud_eval" for now
    pub multi_pv: usize,
    pub base_url: String,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            provider: "cloud_eval".to_string(),
            multi_pv: 5,
            base_url: "https://lichess.org/api/cloud-eval".to_string(),
        }
    }
}
