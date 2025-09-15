use async_trait::async_trait;
use crate::{config::PopularityCfg, infra::Infra, provider::{MovePopularity, PopularityCaps}, domain::{FenKey, PopularityRow}};

/// Lichess Opening Explorer popularity provider.
pub struct Explorer {
    cfg: PopularityCfg,
    infra: Infra,
}

impl Explorer {
    pub fn new(cfg: PopularityCfg, infra: Infra) -> Self { Self { cfg, infra } }
}

#[async_trait]
impl MovePopularity for Explorer {
    async fn sample(&self, fen: &FenKey) -> anyhow::Result<Vec<PopularityRow>> {
        // TODO: use infra.http + infra.rate_explorer + infra.cache to fetch & parse
        // Compute frequencies server-side or return counts & let orchestrator compute rate.
        anyhow::bail!("not implemented: explorer HTTP call")
    }
    fn caps(&self) -> PopularityCaps { PopularityCaps { supports_filters: true } }
}
