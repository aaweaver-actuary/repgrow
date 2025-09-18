use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessSquareParseError;

impl std::fmt::Display for ChessSquareParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid chess square")
    }
}

impl std::error::Error for ChessSquareParseError {}
use crate::domain::chess::{ChessFile, ChessRank};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessSquare(pub ChessFile, pub ChessRank);

impl ChessSquare {
    /// Creates a new `ChessSquare` from the given `ChessFile` and `ChessRank`.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// let square = ChessSquare::new(ChessFile::E, ChessRank::Four);
    /// assert_eq!(square.to_string(), "e4");
    /// ```
    pub fn new(file: ChessFile, rank: ChessRank) -> Self {
        Self(file, rank)
    }

    /// Parses a chess square from a coordinate string like "e4".
    ///
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// use repgrow::domain::chess::chess_square::ChessSquareParseError;
    /// assert_eq!(ChessSquare::from_coords("e4"), Ok(ChessSquare::new(ChessFile::E, ChessRank::Four)));
    /// assert_eq!(ChessSquare::from_coords("a1"), Ok(ChessSquare::new(ChessFile::A, ChessRank::One)));
    /// assert_eq!(ChessSquare::from_coords("h8"), Ok(ChessSquare::new(ChessFile::H, ChessRank::Eight)));
    /// assert_eq!(ChessSquare::from_coords("i9"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::from_coords("e9"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::from_coords("z4"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::from_coords("e"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::from_coords("4"), Err(ChessSquareParseError));
    /// ```
    pub fn from_coords(s: &str) -> Result<Self, ChessSquareParseError> {
        if s.len() != 2 {
            return Err(ChessSquareParseError);
        }
        let mut chars = s.chars();
        let file_char = chars.next().unwrap();
        let rank_char = chars.next().unwrap();
        let file = ChessFile::from_char(file_char).map_err(|_| ChessSquareParseError)?;
        let rank = ChessRank::from_char(rank_char).map_err(|_| ChessSquareParseError)?;
        Ok(Self::new(file, rank))
    }

    /// Converts the `ChessSquare` to its string representation (e.g., "e4").
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// let square = ChessSquare::new(ChessFile::E, ChessRank::Four);
    /// assert_eq!(square.to_coords(), "e4");
    /// let square = ChessSquare::new(ChessFile::A, ChessRank::One);
    /// assert_eq!(square.to_coords(), "a1");
    /// let square = ChessSquare::new(ChessFile::H, ChessRank::Eight);
    /// assert_eq!(square.to_coords(), "h8");
    /// ```
    pub fn to_coords(&self) -> String {
        format!("{}{}", self.0.to_char(), self.1.to_char())
    }

    /// Extract the from square from a UCI move string.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// use repgrow::domain::chess::chess_square::ChessSquareParseError;
    /// assert_eq!(ChessSquare::extract_uci_from_square("e2e4"), Ok(ChessSquare::from_coords("e2").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_from_square("a7a8q"), Ok(ChessSquare::from_coords("a7").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_from_square("h1h8"), Ok(ChessSquare::from_coords("h1").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_from_square("e9e4"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_from_square("i2e4"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_from_square("e2e"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_from_square("e2e4xx"), Err(ChessSquareParseError));
    /// ```
    pub fn extract_uci_from_square(uci: &str) -> Result<ChessSquare, ChessSquareParseError> {
        if uci.len() < 4 || uci.len() > 5 {
            Err(Self::uci_len_format_error(uci))
        } else {
            let from_str = &uci[0..2];
            ChessSquare::from_coords(from_str)
        }
    }

    /// Extract the to square from a UCI move string.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// use repgrow::domain::chess::chess_square::ChessSquareParseError;
    /// assert_eq!(ChessSquare::extract_uci_to_square("e2e4"), Ok(ChessSquare::from_coords("e4").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_to_square("a7a8q"), Ok(ChessSquare::from_coords("a8").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_to_square("h1h8"), Ok(ChessSquare::from_coords("h8").unwrap()));
    /// assert_eq!(ChessSquare::extract_uci_to_square("e2e9"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_to_square("e2i4"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_to_square("e2e"), Err(ChessSquareParseError));
    /// assert_eq!(ChessSquare::extract_uci_to_square("e2e4xx"), Err(ChessSquareParseError));
    /// ```
    pub fn extract_uci_to_square(uci: &str) -> Result<ChessSquare, ChessSquareParseError> {
        if uci.len() < 4 || uci.len() > 5 {
            Err(Self::uci_len_format_error(uci))
        } else {
            let to_str = &uci[2..4];
            ChessSquare::from_coords(to_str)
        }
    }

    fn uci_len_format_error(uci: &str) -> ChessSquareParseError {
        dbg!(format!(
            "Invalid UCI move length: len({})={}",
            uci,
            uci.len()
        ));
        ChessSquareParseError
    }

    /// Returns the file of the chess square.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// let square = ChessSquare::new(ChessFile::E, ChessRank::Four);
    /// assert_eq!(square.file().to_char(), 'e');
    /// assert_eq!(square.file().to_int(), 5);
    /// ```
    pub fn file(&self) -> ChessFile {
        self.0
    }

    /// Returns the rank of the chess square.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::{ChessSquare, ChessFile, ChessRank};
    /// let square = ChessSquare::new(ChessFile::E, ChessRank::Four);
    /// assert_eq!(square.rank().to_int(), 4);
    /// ```
    pub fn rank(&self) -> ChessRank {
        self.1
    }
}

impl std::fmt::Display for ChessSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", self.0.to_char(), self.1.to_char())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_san_valid() {
        ('a'..='h').for_each(|file| {
            ('1'..='8').for_each(|rank| {
                let san = format!("{}{}", file, rank);
                let square = ChessSquare::from_coords(&san).unwrap();
                assert_eq!(square.to_string(), san);
            });
        });
    }

    #[test]
    fn test_from_san_invalid() {
        let invalid_sans = vec!["", "a", "1", "i4", "e9", "e", "44", "ee", "a0", "hh"];
        for san in invalid_sans {
            assert!(ChessSquare::from_coords(san).is_err());
        }
    }

    #[test]
    fn test_to_string() {
        let square = ChessSquare::new(ChessFile::E, ChessRank::Four);
        assert_eq!(square.to_string(), "e4");
        let square = ChessSquare::new(ChessFile::A, ChessRank::One);
        assert_eq!(square.to_string(), "a1");
        let square = ChessSquare::new(ChessFile::H, ChessRank::Eight);
        assert_eq!(square.to_string(), "h8");
    }

    #[test]
    fn test_to_coords() {
        ('a'..='h').for_each(|file| {
            ('1'..='8').for_each(|rank| {
                let f = ChessFile::from_char(file).unwrap();
                let r = ChessRank::from_char(rank).unwrap();
                let square = ChessSquare::new(f, r);
                let san = format!("{}{}", file, rank);
                assert_eq!(square.to_coords(), san);
            });
        });
    }

    #[test]
    fn test_file_and_rank() {
        ('a'..='h').for_each(|file| {
            ('1'..='8').for_each(|rank| {
                let san = format!("{}{}", file, rank);
                let square = ChessSquare::from_coords(&san).unwrap();
                assert_eq!(square.file().to_char(), file);
                assert_eq!(square.rank().to_char(), rank);
            });
        });
    }

    #[test]
    fn test_eq() {
        let sq1 = ChessSquare::new(ChessFile::E, ChessRank::Four);
        let sq2 = ChessSquare::new(ChessFile::E, ChessRank::Four);
        let sq3 = ChessSquare::new(ChessFile::D, ChessRank::Four);
        assert_eq!(sq1, sq2);
        assert_ne!(sq1, sq3);
    }
}
