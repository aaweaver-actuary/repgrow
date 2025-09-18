use crate::domain::CandidateMove;

/// Result of fetching raw candidates from providers (unfiltered).
pub struct RawCandidates {
    pub moves: Vec<CandidateMove>, // normalized (eval/play_rate present when known)
}
