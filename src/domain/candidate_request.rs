use super::fen_key::FenKey;

#[derive(Clone, Debug)]
pub struct CandidateRequest {
    pub fen_key: FenKey,
    pub max_candidates: usize,
    pub cp_window: f32,     // for quality
    pub min_play_rate: f32, // for popularity
    pub multipv: usize,     // for quality
}
