use crate::config::AppConfig;
use crate::infra::cache::MemCache;
use crate::infra::{rate::RateLimiter, singleflight::SingleFlight};
use std::sync::Arc;

pub mod cache;
pub mod http;
pub mod rate;
pub mod scheduler;
pub mod singleflight;

/// Bundle of shared infra for providers/orchestrator.
#[derive(Clone, Debug)]
pub struct Infra {
    pub cache_fen: Arc<MemCache<String, serde_json::Value>>, // example: cache raw blobs by FEN
    pub single: Arc<SingleFlight<String, serde_json::Value>>,
    pub rate_cloud: RateLimiter,
    pub rate_explorer: RateLimiter,
    pub http: reqwest::Client,
    pub sched: scheduler::TokioScheduler,
}

pub fn build_infra(cfg: &AppConfig) -> anyhow::Result<Infra> {
    let cache_fen = Arc::new(MemCache::new(cfg.cache.entries, cfg.cache.ttl_secs));
    let single = Arc::new(SingleFlight::new());
    Ok(Infra {
        cache_fen,
        single,
        rate_cloud: RateLimiter::per_sec(cfg.rate.cloud_per_sec),
        rate_explorer: RateLimiter::per_sec(cfg.rate.explorer_per_sec),
        http: http::build_http(cfg.http.timeout_ms),
        sched: scheduler::TokioScheduler,
    })
}
