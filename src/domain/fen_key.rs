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
    /// Creates a new FenKey from a FEN string and side to move.
    /// # Arguments
    /// * `fen_string` - The FEN string representing the board position.
    /// * `side_to_move` - The side to move (PieceColor).
    /// # Returns
    /// * `FenKey` - A new FenKey instance.
    /// # Examples
    /// ```
    /// use repgrow::domain::{FenKey, PieceColor};
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    /// let key = FenKey::new(fen.clone(), PieceColor::White);
    /// assert_eq!(key.fen_string, fen);
    /// assert_eq!(key.side_to_move, PieceColor::White);
    /// ```
    pub fn new(fen_string: String, side_to_move: PieceColor) -> Self {
        Self {
            fen_string,
            side_to_move,
        }
    }

    /// Returns a FenKey representing the standard starting position.
    /// # Returns
    /// * `FenKey` - A FenKey for the starting position.
    /// # Examples
    /// ```
    /// use repgrow::domain::{FenKey, PieceColor};
    /// let starting_key = FenKey::starting_position();
    /// assert_eq!(starting_key.fen_string, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    /// assert_eq!(starting_key.side_to_move, PieceColor::White);
    /// ```
    pub fn starting_position() -> Self {
        Self {
            fen_string: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            side_to_move: PieceColor::White,
        }
    }
}

impl std::fmt::Display for FenKey {
    /// Formats the FenKey as "FEN side_to_move".
    /// # Examples
    /// ```
    /// use repgrow::domain::{FenKey, PieceColor};
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    /// let key = FenKey::new(fen.clone(), PieceColor::White);
    /// assert_eq!(format!("{}", key), format!("{} {}", fen, "white"));
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.fen_string, self.side_to_move)
    }
}
