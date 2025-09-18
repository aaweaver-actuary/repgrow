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
    use crate::domain::{Centipawns, PieceColor, PlayRate};

    use super::*;

    #[test]
    fn test_candidate_move_creation() {
        let move_ = CandidateMove {
            uci: "e2e4".to_string(),
            next_fen: FenKey {
                fen_string: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
                    .to_string(),
                side_to_move: PieceColor::Black,
            },
            signals: Signals {
                eval_cp: Some(Centipawns::from_float(0.85)),
                depth: None,
                play_rate: Some(PlayRate::new(0.75)),
                games: None,
            },
        };

        assert_eq!(move_.uci, "e2e4");
        assert_eq!(
            move_.next_fen.fen_string,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );
        assert_eq!(move_.next_fen.side_to_move, PieceColor::Black);
        assert_eq!(move_.signals.play_rate, Some(PlayRate::new(0.75)));
        assert_eq!(move_.signals.eval_cp, Some(Centipawns::from_float(0.85)));
    }
}
