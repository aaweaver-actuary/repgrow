pub mod cloud_eval;
pub mod explorer;
pub mod popularity;
pub mod popularity_caps;
pub mod quality;
pub mod quality_caps;
pub mod types;

pub use cloud_eval::CloudEval;
pub use explorer::Explorer;
pub use popularity_caps::PopularityCaps;
pub use quality_caps::QualityCaps;
pub use types::CandidateMoves;

use crate::{
    config::{PopularityCfg, QualityCfg},
    domain::{CandidateMove, EvalLine, FenKey, PopularityRow, Signals},
    infra::Infra,
    provider::types::EvalLines,
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait MoveQuality: Send + Sync {
    async fn evaluate(&self, fen: &FenKey, multipv: usize) -> anyhow::Result<Vec<EvalLine>>;
    fn caps(&self) -> QualityCaps;
}

#[async_trait]
pub trait MovePopularity: Send + Sync {
    async fn sample(&self, fen: &FenKey) -> anyhow::Result<Vec<PopularityRow>>;
    fn caps(&self) -> PopularityCaps;
}

/// Factory: late-bind providers from config.
pub fn build_quality(cfg: &QualityCfg, infra: &Infra) -> anyhow::Result<Arc<dyn MoveQuality>> {
    match cfg.provider.as_str() {
        "cloud_eval" => Ok(Arc::new(CloudEval::new(cfg.clone(), infra.clone()))),
        other => anyhow::bail!("unknown quality provider '{other}'"),
    }
}
pub fn build_popularity(
    cfg: &PopularityCfg,
    infra: &Infra,
) -> anyhow::Result<Arc<dyn MovePopularity>> {
    match cfg.provider.as_str() {
        "explorer" => Ok(Arc::new(Explorer::new(cfg.clone(), infra.clone()))),
        other => anyhow::bail!("unknown popularity provider '{other}'"),
    }
}

/// Normalize specialized outputs into unified CandidateMove.
pub fn normalize_quality(fen: &FenKey, lines: EvalLines) -> CandidateMoves {
    lines
        .into_iter()
        .map(|l| {
            let mut sig = Signals::default();
            sig.eval_cp = Some(l.eval_cp as f32);
            sig.depth = Some(l.depth);
            // next_fen is filled by orchestrator using shakmaty (legal move application)
            CandidateMove {
                uci: l.uci,
                next_fen: fen.clone(),
                signals: sig,
            }
        })
        .collect()
}
pub fn normalize_popularity(fen: &FenKey, rows: Vec<PopularityRow>) -> CandidateMoves {
    rows.into_iter()
        .map(|r| {
            let mut sig = Signals::default();
            sig.play_rate = Some(r.play_rate);
            sig.games = Some(r.games);
            CandidateMove {
                uci: r.uci,
                next_fen: fen.clone(),
                signals: sig,
            }
        })
        .collect()
}
