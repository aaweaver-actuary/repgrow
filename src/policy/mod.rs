pub mod decision;
pub mod split_side_policy;

pub use decision::Decision;
pub use split_side_policy::SideSplitPolicy;

use crate::domain::CandidateRequest;
use crate::provider::types::CandidateMoves;
use shakmaty::Color;

/// Policy decides role and request shaping, and can post-filter.
pub trait MovePolicy: Send + Sync {
    fn decide(&self, stm: Color) -> Decision;
    fn adjust(&self, req: &mut CandidateRequest, is_my_side: bool);
    fn post_filter(&self, mut cands: CandidateMoves) -> CandidateMoves {
        // Stable ordering: primary → secondary → UCI for determinism
        cands.sort_by(|a, b| {
            let pa_eval = a.signals.eval_cp.unwrap_or(-10000.0);
            let pb_eval = b.signals.eval_cp.unwrap_or(-10000.0);

            match pa_eval
                .partial_cmp(&pb_eval)
                .unwrap_or(std::cmp::Ordering::Equal)
            {
                std::cmp::Ordering::Equal => {
                    let pa_play = a.signals.play_rate.unwrap_or(-1.0);
                    let pb_play = b.signals.play_rate.unwrap_or(-1.0);
                    match pa_play
                        .partial_cmp(&pb_play)
                        .unwrap_or(std::cmp::Ordering::Equal)
                    {
                        std::cmp::Ordering::Equal => a.uci.cmp(&b.uci),
                        other => other,
                    }
                }
                other => other,
            }
        });
        cands
    }
}
