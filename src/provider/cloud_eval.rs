use crate::{
    config::QualityCfg,
    domain::{EvalLine, FenKey},
    infra::Infra,
    provider::{MoveQuality, QualityCaps},
};
use async_trait::async_trait;
use std::sync::Arc;

/// Lichess Cloud Evaluation provider (engine-quality).
/// Talks to /api/cloud-eval and returns MultiPV lines.
pub struct CloudEval {
    cfg: QualityCfg,
    infra: Infra,
}

impl CloudEval {
    pub fn new(cfg: QualityCfg, infra: Infra) -> Self {
        Self { cfg, infra }
    }
}

#[async_trait]
impl MoveQuality for CloudEval {
    async fn evaluate(&self, fen: &FenKey, multipv: usize) -> anyhow::Result<Vec<EvalLine>> {
        // TODO: use infra.http + infra.rate_cloud + infra.cache to fetch & parse
        // Return Vec<EvalLine> with (uci, eval_cp, depth)
        anyhow::bail!("not implemented: cloud-eval HTTP call")
    }
    fn caps(&self) -> QualityCaps {
        QualityCaps {
            max_multipv: self.cfg.cloud_multi_pv,
        }
    }
}
