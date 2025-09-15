use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub search: SearchCfg,
    pub policy: PolicyCfg,
    pub quality: QualityCfg,
    pub popularity: PopularityCfg,
    pub http: HttpCfg,
    pub cache: CacheCfg,
    pub rate: RateCfg,
}

impl AppConfig {
    /// Load config from a TOML file.
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let s = fs::read_to_string(path)?;
        Ok(toml::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchCfg {
    pub concurrency: usize,
    pub max_total_nodes: usize,
    pub max_children_my_side: usize,
    pub max_children_opp_side: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyCfg {
    pub my_side: Option<String>,
    pub cp_window: i32,
    pub min_play_rate: f32,
}

impl PolicyCfg {
    pub fn resolve_side_override(&self, cli_side: &str) -> anyhow::Result<shakmaty::Color> {
        let s = if !cli_side.is_empty() { Some(cli_side.to_string()) } else { self.my_side.clone() };
        match s.as_deref() {
            Some("white") => Ok(shakmaty::Color::White),
            Some("black") => Ok(shakmaty::Color::Black),
            _ => anyhow::bail!("side must be white|black"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct QualityCfg {
    pub provider: String,            // "cloud_eval" for now
    pub cloud_multi_pv: usize,
    pub cloud_base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PopularityCfg {
    pub provider: String,            // "explorer" for now
    pub base_url: String,
    pub speed: String,
    pub min_rating: u32,
    pub max_rating: u32,
    pub since_year: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpCfg {
    pub timeout_ms: u64,
    pub retries: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CacheCfg {
    pub entries: u64,
    pub ttl_secs: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateCfg {
    pub cloud_per_sec: u32,
    pub explorer_per_sec: u32,
}
