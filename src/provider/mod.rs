pub mod cloud_eval;
pub mod explorer;
pub mod move_popularity;
pub mod move_quality;
pub mod popularity;
pub mod popularity_caps;
pub mod quality;
pub mod quality_caps;
pub mod types;

pub use cloud_eval::LichessEvalClient;
pub use explorer::Explorer;
pub use move_popularity::MovePopularity;
pub use move_quality::MoveQuality;
pub use popularity_caps::PopularityCaps;
pub use quality_caps::QualityCaps;
pub use types::CandidateMoves;

use crate::{
    config::{PopularityConfig, QualityConfig},
    domain::{CandidateMove, EvalLine, FenKey, PlayRate, PopularityRow, Signals},
    infra::Infra,
    provider::{cloud_eval::build_lichess_eval_client, types::EvalLines},
};
use std::sync::Arc;

/// Factory: late-bind providers from config.
pub fn build_quality(cfg: &QualityConfig, _infra: &Infra) -> anyhow::Result<Arc<dyn MoveQuality>> {
    let client = build_lichess_eval_client(&cfg.base_url, cfg.multi_pv, cfg.clone());
    match cfg.source.as_str() {
        "cloud_eval" => Ok(Arc::new(client)),
        other => anyhow::bail!("unknown quality provider '{other}'"),
    }
}
pub fn build_popularity(
    cfg: &PopularityConfig,
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
            sig.play_rate = Some(PlayRate::new(r.play_rate));
            sig.games = Some(r.games);
            CandidateMove {
                uci: r.uci,
                next_fen: fen.clone(),
                signals: sig,
            }
        })
        .collect()
}
