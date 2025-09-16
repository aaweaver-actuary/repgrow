use crate::domain::PieceColor;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Result};

/// FEN + side-to-move forms a canonical lookup key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FenKey {
    pub fen_string: String,
    pub side_to_move: PieceColor,
}

impl FenKey {
    pub fn new(fen_string: String, side_to_move: PieceColor) -> Self {
        Self {
            fen_string,
            side_to_move,
        }
    }

    pub fn starting_position() -> Self {
        Self {
            fen_string: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            side_to_move: PieceColor::White,
        }
    }
}

impl std::fmt::Display for FenKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.fen_string, self.side_to_move)
    }
}
