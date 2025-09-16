use crate::{
    config::PopularityConfig,
    domain::{FenKey, PopularityRow},
    infra::Infra,
    provider::{MovePopularity, PopularityCaps},
};
use async_trait::async_trait;

/// Lichess Opening Explorer popularity provider.
#[derive(Debug, Clone)]

pub struct Explorer {
    cfg: PopularityConfig,
    infra: Infra,
}

impl Explorer {
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Explorer")
    }
}

impl Explorer {
    pub fn new(cfg: PopularityConfig, infra: Infra) -> Self {
        Self { cfg, infra }
    }
}

#[async_trait]
impl MovePopularity for Explorer {
    async fn sample(&self, _fen: &FenKey) -> anyhow::Result<Vec<PopularityRow>> {
        // TODO: use infra.http + infra.rate_explorer + infra.cache to fetch & parse
        // Compute frequencies server-side or return counts & let orchestrator compute rate.
        anyhow::bail!("not implemented: explorer HTTP call")
    }
    fn caps(&self) -> PopularityCaps {
        PopularityCaps {
            supports_filters: true,
        }
    }
}
