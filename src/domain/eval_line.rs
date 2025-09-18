use serde::{Deserialize, Serialize};

use crate::domain::{Centipawns, chess::UciMove};

/// Convenience struct returned by specialized providers before normalization.
/// EvalLine represents a single move evaluation from a chess engine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvalLine {
    pub uci: UciMove,
    pub eval_cp: Centipawns,
    pub depth: u8,
}
