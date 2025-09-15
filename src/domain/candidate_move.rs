use super::fen_key::FenKey;
use super::signals::Signals;

#[derive(Clone, Debug)]
pub struct CandidateMove {
    pub uci: String,
    pub next_fen: FenKey,
    pub signals: Signals,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_move_creation() {
        let move_ = CandidateMove {
            uci: "e2e4".to_string(),
            next_fen: FenKey {
                fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string(),
                stm: crate::domain::PieceColor::Black,
            },
            signals: Signals {
                popularity: 0.75,
                quality: 0.85,
            },
        };

        assert_eq!(move_.uci, "e2e4");
        assert_eq!(
            move_.next_fen.fen,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
        assert_eq!(move_.next_fen.stm, crate::domain::PieceColor::Black);
        assert_eq!(move_.signals.popularity, 0.75);
        assert_eq!(move_.signals.quality, 0.85);
    }
}
