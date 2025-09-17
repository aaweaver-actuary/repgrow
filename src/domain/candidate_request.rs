use super::fen_key::FenKey;
use crate::domain::{Centipawns, PlayRate};
use derive_builder::Builder;

#[derive(Clone, Debug, Builder)]
pub struct CandidateRequest {
    #[builder(default = "FenKey::starting_position()")]
    pub fen_key: FenKey,
    #[builder(default = "5")]
    pub max_candidates: usize,
    #[builder(default = "Centipawns::from_int(0)")]
    pub cp_window: Centipawns,
    #[builder(default = "PlayRate::new(0.0)")]
    pub min_play_rate: PlayRate,
    #[builder(default = "1")]
    pub multipv: usize,
}

impl CandidateRequest {
    /// Create a new CandidateRequest with specified parameters.
    /// # Arguments
    /// * `fen_key` - The FenKey representing the position.
    /// * `max_candidates` - Maximum number of candidate moves to request.
    /// * `cp_window` - Centipawn window for move quality filtering.
    /// * `min_play_rate` - Minimum play rate for move popularity filtering.
    /// * `multipv` - Number of principal variations to request from the engine.
    /// # Returns
    /// * `CandidateRequest` - The constructed CandidateRequest.
    /// # Examples
    /// ```
    /// use repgrow::domain::{CandidateRequest, Centipawns, PlayRate};
    /// use repgrow::domain::FenKey;
    /// let fen_key = FenKey::starting_position();
    /// let req = CandidateRequest::new(fen_key, 10, Centipawns::from_int(50), PlayRate::new(0.05), 3);
    /// assert_eq!(req.fen_key.fen_string, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    /// assert_eq!(req.fen_key.side_to_move.to_string(), "white".to_string());
    /// assert_eq!(req.max_candidates, 10);
    /// assert_eq!(req.cp_window, Centipawns::from_int(50));
    /// assert_eq!(req.min_play_rate, PlayRate::new(0.05));
    /// assert_eq!(req.multipv, 3);
    /// ```
    pub fn new(
        fen_key: FenKey,
        max_candidates: usize,
        cp_window: Centipawns,
        min_play_rate: PlayRate,
        multipv: usize,
    ) -> Self {
        Self {
            fen_key,
            max_candidates,
            cp_window,
            min_play_rate,
            multipv,
        }
    }
}
