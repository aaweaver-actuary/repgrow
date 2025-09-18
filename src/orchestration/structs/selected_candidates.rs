use crate::domain::CandidateMove;

/// Result after policy filtering/capping and deterministic ordering.
pub struct SelectedCandidates {
    pub moves: Vec<CandidateMove>,
}
