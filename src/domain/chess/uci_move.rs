use crate::domain::chess::{ChessPieceType, ChessSquare};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UciMove {
    pub from: ChessSquare,
    pub to: ChessSquare,
    pub promotion: Option<ChessPieceType>,
}

/// Error type for UCI move parsing failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UciMoveParseError;

impl UciMove {
    /// Creates a new UciMove.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{UciMove, ChessSquare, ChessFile, ChessRank, ChessPieceType};
    /// let from_sq = ChessSquare::new(ChessFile::E, ChessRank::Two);
    /// let to_sq = ChessSquare::new(ChessFile::E, ChessRank::Four);
    /// let uci_move = UciMove::new(from_sq, to_sq, None);
    /// assert_eq!(uci_move.from, from_sq);
    /// assert_eq!(uci_move.to, to_sq);
    /// assert_eq!(uci_move.promotion, None);
    /// ```
    pub fn new(from: ChessSquare, to: ChessSquare, promotion: Option<ChessPieceType>) -> Self {
        Self {
            from,
            to,
            promotion,
        }
    }

    /// Parses a UCI move string (e.g., "e2e4", "e7e8q") into a `UciMove`.
    ///
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{UciMove, ChessSquare, ChessFile, ChessRank, ChessPieceType};
    /// use repgrow::domain::chess::uci_move::UciMoveParseError;
    /// let from_sq = ChessSquare::from_coords("e2").unwrap();
    /// let to_sq = ChessSquare::from_coords("e4").unwrap();
    /// let uci_move = UciMove::new(from_sq, to_sq, None);
    /// assert_eq!(UciMove::from_uci("e2e4"), Ok(uci_move));
    /// assert_eq!(UciMove::from_uci("e7e8q"), Ok(UciMove {
    ///     from: ChessSquare::new(ChessFile::E, ChessRank::Seven),
    ///     to: ChessSquare::new(ChessFile::E, ChessRank::Eight),
    ///     promotion: Some(ChessPieceType::Queen),
    /// }));
    /// assert_eq!(UciMove::from_uci("a7a8n"), Ok(UciMove {
    ///     from: ChessSquare::new(ChessFile::A, ChessRank::Seven),
    ///     to: ChessSquare::new(ChessFile::A, ChessRank::Eight),
    ///     promotion: Some(ChessPieceType::Knight),
    /// }));
    /// ```
    pub fn from_uci(s: &str) -> Result<Self, UciMoveParseError> {
        if s.len() < 4 || s.len() > 5 {
            return Err(UciMoveParseError);
        }
        let from_square =
            ChessSquare::extract_uci_from_square(s).expect("Failed to extract from square");
        let to_square = ChessSquare::extract_uci_to_square(s).expect("Failed to extract to square");
        let promotion_piece = ChessPieceType::from_uci_string(s);

        // Validate promotion rules
        if let Some(promo) = promotion_piece {
            // Promotion can only occur when moving to the last rank
            let is_white_pawn = from_square.rank().to_int() == 7 && to_square.rank().to_int() == 8;
            let is_black_pawn = from_square.rank().to_int() == 2 && to_square.rank().to_int() == 1;
            let is_pawn_promotion = is_white_pawn || is_black_pawn;
            if !is_pawn_promotion {
                let err_msg = format!(
                    "Invalid promotion move: {:?} to {:?} with promotion to {:?}.\nPromotion can only occur when a pawn reaches the last rank, so it must start from one rank before.",
                    from_square.to_coords(),
                    to_square.to_coords(),
                    promo
                );
                eprintln!("{}", err_msg);
                return Err(UciMoveParseError);
            }

            // Ensure the piece being promoted to is valid
            match promo {
                ChessPieceType::Queen
                | ChessPieceType::Rook
                | ChessPieceType::Bishop
                | ChessPieceType::Knight => {}
                _ => return Err(UciMoveParseError),
            }

            Ok(UciMove::new(from_square, to_square, Some(promo)))
        } else {
            Ok(UciMove::new(from_square, to_square, None))
        }
    }

    /// Converts the UciMove back to its UCI string representation.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{UciMove, ChessSquare, ChessFile, ChessRank, ChessPieceType};
    /// let uci_move = UciMove::new(
    ///     ChessSquare::new(ChessFile::E, ChessRank::Two),
    ///     ChessSquare::new(ChessFile::E, ChessRank::Four),
    ///     None,
    /// );
    /// assert_eq!(uci_move.to_uci(), "e2e4");
    /// let uci_with_promo = UciMove::new(
    ///     ChessSquare::new(ChessFile::E, ChessRank::Seven),
    ///     ChessSquare::new(ChessFile::E, ChessRank::Eight),
    ///     Some(ChessPieceType::Queen),
    /// );
    /// assert_eq!(uci_with_promo.to_uci(), "e7e8q");
    /// ```
    pub fn to_uci(&self) -> String {
        let mut uci_str = format!("{}{}", self.from.to_coords(), self.to.to_coords());
        if let Some(promo) = self.promotion {
            uci_str.push(promo.to_char().to_ascii_lowercase());
        }
        uci_str
    }
}
