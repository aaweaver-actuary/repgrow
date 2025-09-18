use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChessPieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPieceType {
    /// Creates a ChessPieceType from a character ('p','n','b','r','q','k' or uppercase).
    /// Returns None for characters outside this range.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessPieceType;
    /// assert_eq!(ChessPieceType::from_char('p'), Some(ChessPieceType::Pawn));
    /// assert_eq!(ChessPieceType::from_char('K'), Some(ChessPieceType::King));
    /// assert_eq!(ChessPieceType::from_char('x'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'p' => Some(ChessPieceType::Pawn),
            'n' => Some(ChessPieceType::Knight),
            'b' => Some(ChessPieceType::Bishop),
            'r' => Some(ChessPieceType::Rook),
            'q' => Some(ChessPieceType::Queen),
            'k' => Some(ChessPieceType::King),
            _ => None,
        }
    }

    /// Converts the ChessPieceType to its corresponding character used in SAN ('p','N','B','R','Q','K').
    ///
    /// # Notes
    /// This function is used to convert the internal representation of a chess piece type
    /// to the character representation used in Standard Algebraic Notation (SAN). In SAN,
    /// pawns are represented by the absence of a letter, but for the purpose of this
    /// function, we use 'p' to represent pawns for consistency.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessPieceType;
    /// assert_eq!(ChessPieceType::Pawn.to_char(), 'p');
    /// assert_eq!(ChessPieceType::King.to_char(), 'K');
    /// ```
    pub fn to_char(self) -> char {
        match self {
            ChessPieceType::Pawn => 'p',
            ChessPieceType::Knight => 'N',
            ChessPieceType::Bishop => 'B',
            ChessPieceType::Rook => 'R',
            ChessPieceType::Queen => 'Q',
            ChessPieceType::King => 'K',
        }
    }

    /// Extracts the promotion piece type from a UCI promotion string.
    /// Returns None if the character does not correspond to a valid promotion piece type, or if it is not included.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessPieceType;
    /// assert_eq!(ChessPieceType::from_uci_string("e7e8q"), Some(ChessPieceType::Queen));
    /// assert_eq!(ChessPieceType::from_uci_string("e7e8n"), Some(ChessPieceType::Knight));
    /// assert_eq!(ChessPieceType::from_uci_string("e7e8"), None);
    /// assert_eq!(ChessPieceType::from_uci_string("e7e8x"), None);
    /// ```
    pub fn from_uci_string(uci: &str) -> Option<Self> {
        if uci.len() == 5 {
            let promo_char = uci.chars().nth(4).unwrap();
            Self::from_char(promo_char)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_char_works_for_lowercase_chars() {
        assert_eq!(ChessPieceType::from_char('p'), Some(ChessPieceType::Pawn));
        assert_eq!(ChessPieceType::from_char('n'), Some(ChessPieceType::Knight));
        assert_eq!(ChessPieceType::from_char('b'), Some(ChessPieceType::Bishop));
        assert_eq!(ChessPieceType::from_char('r'), Some(ChessPieceType::Rook));
        assert_eq!(ChessPieceType::from_char('q'), Some(ChessPieceType::Queen));
        assert_eq!(ChessPieceType::from_char('k'), Some(ChessPieceType::King));
    }

    #[test]
    fn test_from_char_works_for_uppercase_chars() {
        assert_eq!(ChessPieceType::from_char('P'), Some(ChessPieceType::Pawn));
        assert_eq!(ChessPieceType::from_char('N'), Some(ChessPieceType::Knight));
        assert_eq!(ChessPieceType::from_char('B'), Some(ChessPieceType::Bishop));
        assert_eq!(ChessPieceType::from_char('R'), Some(ChessPieceType::Rook));
        assert_eq!(ChessPieceType::from_char('Q'), Some(ChessPieceType::Queen));
        assert_eq!(ChessPieceType::from_char('K'), Some(ChessPieceType::King));
    }

    #[test]
    fn test_from_char_returns_none_for_invalid_chars() {
        assert_eq!(ChessPieceType::from_char('1'), None);
        assert_eq!(ChessPieceType::from_char('x'), None);
    }

    #[test]
    fn test_to_char_returns_correct_char() {
        assert_eq!(ChessPieceType::Pawn.to_char(), 'p');
        assert_eq!(ChessPieceType::Knight.to_char(), 'N');
        assert_eq!(ChessPieceType::Bishop.to_char(), 'B');
        assert_eq!(ChessPieceType::Rook.to_char(), 'R');
        assert_eq!(ChessPieceType::Queen.to_char(), 'Q');
        assert_eq!(ChessPieceType::King.to_char(), 'K');
    }
}
