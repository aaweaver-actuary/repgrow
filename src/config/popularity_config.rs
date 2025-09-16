use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PopularityConfig {
    pub provider: String, // "explorer" for now
    pub base_url: String,
    pub speed: String,
    pub min_rating: u32,
    pub max_rating: u32,
    pub since_year: u32,
}
