use serde::{Deserialize, Serialize};

/// Convenience struct returned by specialized providers before normalization.
/// EvalLine represents a single move evaluation from a chess engine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvalLine {
    pub uci: String,
    pub eval_cp: i32,
    pub depth: u8,
}
