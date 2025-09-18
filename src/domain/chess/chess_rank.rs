use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessRankParseError;

impl std::fmt::Display for ChessRankParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid chess rank")
    }
}

impl std::error::Error for ChessRankParseError {}
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChessRank {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl ChessRank {
    /// Converts a character to a `ChessRank`.
    ///
    /// Accepts characters '1' through '8' and returns the corresponding `ChessRank`.
    /// Returns an error for invalid characters.
    pub fn from_char(c: char) -> Result<ChessRank, ChessRankParseError> {
        match c {
            '1' => Ok(ChessRank::One),
            '2' => Ok(ChessRank::Two),
            '3' => Ok(ChessRank::Three),
            '4' => Ok(ChessRank::Four),
            '5' => Ok(ChessRank::Five),
            '6' => Ok(ChessRank::Six),
            '7' => Ok(ChessRank::Seven),
            '8' => Ok(ChessRank::Eight),
            _ => Err(ChessRankParseError),
        }
    }

    /// Converts an integer (1-8) to a `ChessRank`. Returns an error for invalid integers.
    pub fn from_int(n: u8) -> Result<ChessRank, ChessRankParseError> {
        match n {
            1 => Ok(ChessRank::One),
            2 => Ok(ChessRank::Two),
            3 => Ok(ChessRank::Three),
            4 => Ok(ChessRank::Four),
            5 => Ok(ChessRank::Five),
            6 => Ok(ChessRank::Six),
            7 => Ok(ChessRank::Seven),
            8 => Ok(ChessRank::Eight),
            _ => Err(ChessRankParseError),
        }
    }

    /// Converts the `ChessRank` to its corresponding character ('1'-'8').
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessRank;
    /// assert_eq!(ChessRank::One.to_char(), '1');
    /// assert_eq!(ChessRank::Five.to_char(), '5');
    /// ```
    pub fn to_char(&self) -> char {
        match self {
            ChessRank::One => '1',
            ChessRank::Two => '2',
            ChessRank::Three => '3',
            ChessRank::Four => '4',
            ChessRank::Five => '5',
            ChessRank::Six => '6',
            ChessRank::Seven => '7',
            ChessRank::Eight => '8',
        }
    }

    /// Converts the `ChessRank` to its corresponding integer (1-8).
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessRank;
    /// assert_eq!(ChessRank::One.to_int(), 1);
    /// assert_eq!(ChessRank::Five.to_int(), 5);
    /// ```
    pub fn to_int(&self) -> u8 {
        match self {
            ChessRank::One => 1,
            ChessRank::Two => 2,
            ChessRank::Three => 3,
            ChessRank::Four => 4,
            ChessRank::Five => 5,
            ChessRank::Six => 6,
            ChessRank::Seven => 7,
            ChessRank::Eight => 8,
        }
    }
}

impl Display for ChessRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_char_works_for_valid_chars() {
        assert_eq!(ChessRank::from_char('1'), Ok(ChessRank::One));
        assert_eq!(ChessRank::from_char('2'), Ok(ChessRank::Two));
        assert_eq!(ChessRank::from_char('3'), Ok(ChessRank::Three));
        assert_eq!(ChessRank::from_char('4'), Ok(ChessRank::Four));
        assert_eq!(ChessRank::from_char('5'), Ok(ChessRank::Five));
        assert_eq!(ChessRank::from_char('6'), Ok(ChessRank::Six));
        assert_eq!(ChessRank::from_char('7'), Ok(ChessRank::Seven));
        assert_eq!(ChessRank::from_char('8'), Ok(ChessRank::Eight));
    }

    #[test]
    fn test_from_char_returns_err_for_invalid_chars() {
        assert_eq!(ChessRank::from_char('0'), Err(ChessRankParseError));
        assert_eq!(ChessRank::from_char('9'), Err(ChessRankParseError));
        assert_eq!(ChessRank::from_char('a'), Err(ChessRankParseError));
    }

    #[test]
    fn test_from_int_works_for_valid_ints() {
        assert_eq!(ChessRank::from_int(1), Ok(ChessRank::One));
        assert_eq!(ChessRank::from_int(2), Ok(ChessRank::Two));
        assert_eq!(ChessRank::from_int(3), Ok(ChessRank::Three));
        assert_eq!(ChessRank::from_int(4), Ok(ChessRank::Four));
        assert_eq!(ChessRank::from_int(5), Ok(ChessRank::Five));
        assert_eq!(ChessRank::from_int(6), Ok(ChessRank::Six));
        assert_eq!(ChessRank::from_int(7), Ok(ChessRank::Seven));
        assert_eq!(ChessRank::from_int(8), Ok(ChessRank::Eight));
    }

    #[test]
    fn test_from_int_returns_err_for_invalid_ints() {
        assert_eq!(ChessRank::from_int(0), Err(ChessRankParseError));
        assert_eq!(ChessRank::from_int(9), Err(ChessRankParseError));
        assert_eq!(ChessRank::from_int(10), Err(ChessRankParseError));
    }

    #[test]
    fn test_to_char_returns_correct_char() {
        assert_eq!(ChessRank::One.to_char(), '1');
        assert_eq!(ChessRank::Two.to_char(), '2');
        assert_eq!(ChessRank::Three.to_char(), '3');
        assert_eq!(ChessRank::Four.to_char(), '4');
        assert_eq!(ChessRank::Five.to_char(), '5');
        assert_eq!(ChessRank::Six.to_char(), '6');
        assert_eq!(ChessRank::Seven.to_char(), '7');
        assert_eq!(ChessRank::Eight.to_char(), '8');
    }

    #[test]
    fn test_to_int_returns_correct_int() {
        assert_eq!(ChessRank::One.to_int(), 1);
        assert_eq!(ChessRank::Two.to_int(), 2);
        assert_eq!(ChessRank::Three.to_int(), 3);
        assert_eq!(ChessRank::Four.to_int(), 4);
        assert_eq!(ChessRank::Five.to_int(), 5);
        assert_eq!(ChessRank::Six.to_int(), 6);
        assert_eq!(ChessRank::Seven.to_int(), 7);
        assert_eq!(ChessRank::Eight.to_int(), 8);
    }

    #[test]
    fn test_display_works() {
        assert_eq!(format!("{}", ChessRank::One), "1");
        assert_eq!(format!("{}", ChessRank::Two), "2");
        assert_eq!(format!("{}", ChessRank::Three), "3");
        assert_eq!(format!("{}", ChessRank::Four), "4");
        assert_eq!(format!("{}", ChessRank::Five), "5");
        assert_eq!(format!("{}", ChessRank::Six), "6");
        assert_eq!(format!("{}", ChessRank::Seven), "7");
        assert_eq!(format!("{}", ChessRank::Eight), "8");
    }
}
