pub mod candidate_move;
pub mod candidate_request;
pub mod color;
pub mod fen_key;
pub mod repertoire_node;
pub mod signals;

pub use candidate_move::CandidateMove;
pub use candidate_request::CandidateRequest;
pub use color::PieceColor;
pub use fen_key::FenKey;
pub use repertoire_node::RepertoireNode;
pub use signals::Signals;

/// Convenience structs returned by specialized providers before normalization.
#[derive(Clone, Debug)]
pub struct EvalLine {
    pub uci: String,
    pub eval_cp: i32,
    pub depth: u8,
}
#[derive(Clone, Debug)]
pub struct PopularityRow {
    pub uci: String,
    pub play_rate: f32,
    pub games: u32,
}
