pub mod decision;
pub mod split_side_policy;

pub use decision::Decision;
pub use split_side_policy::SideSplitPolicy;

use std::cmp::Ordering;

use crate::domain::{CandidateRequest, Centipawns, PlayRate};
use crate::provider::types::CandidateMoves;
use shakmaty::Color;

/// Policy decides role and request shaping, and can post-filter.
pub trait MovePolicy: Send + Sync {
    /// Decide role (attacker/defender) for current side to move.
    fn decide(&self, stm: Color) -> Decision;

    /// Adjust request (e.g. multipv) based on role and side.
    /// is_my_side is true if stm matches the side this policy is for.
    fn adjust(&self, req: &mut CandidateRequest, is_my_side: bool);

    /// Post-filter candidate moves (e.g. sort, trim) before returning to orchestrator.
    /// Candidates have signals from all providers merged in, so can be sorted/filtered.
    /// Default implementation sorts by eval_cp desc, then play_rate desc, then UCI asc
    fn post_filter(&self, mut cands: CandidateMoves) -> CandidateMoves {
        // Stable ordering: primary → secondary → UCI for determinism
        cands.sort_by(|a, b| {
            let pa_eval = a.signals.eval_cp.unwrap_or(Centipawns::from_int(-10000));
            let pb_eval = b.signals.eval_cp.unwrap_or(Centipawns::from_int(-10000));

            match pa_eval.partial_cmp(&pb_eval).unwrap_or(Ordering::Equal) {
                Ordering::Equal => {
                    let pa_play = a.signals.play_rate.unwrap_or(PlayRate::new(-1.0));
                    let pb_play = b.signals.play_rate.unwrap_or(PlayRate::new(-1.0));
                    let str_a = format!("{}{}", a.uci.from.to_coords(), a.uci.to.to_coords());
                    let str_b = format!("{}{}", b.uci.from.to_coords(), b.uci.to.to_coords());
                    match pa_play.compare(&pb_play) {
                        Ordering::Equal => str_a.cmp(&str_b),
                        other => other,
                    }
                }
                other => other,
            }
        });
        cands
    }
}
