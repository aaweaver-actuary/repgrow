use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChessFileParseError;

impl std::fmt::Display for ChessFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid chess file")
    }
}

impl std::error::Error for ChessFileParseError {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChessFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl ChessFile {
    /// Converts the ChessFile to its corresponding u8 value (1-8).
    /// A -> 1, B -> 2, ..., H -> 8
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessFile;
    /// assert_eq!(ChessFile::A.to_u8(), 1);
    /// assert_eq!(ChessFile::H.to_u8(), 8);
    /// ```
    pub fn to_u8(&self) -> u8 {
        match self {
            ChessFile::A => 1,
            ChessFile::B => 2,
            ChessFile::C => 3,
            ChessFile::D => 4,
            ChessFile::E => 5,
            ChessFile::F => 6,
            ChessFile::G => 7,
            ChessFile::H => 8,
        }
    }

    /// Alias for to_u8
    pub fn to_int(&self) -> u8 {
        self.to_u8()
    }

    /// Converts the ChessFile to its corresponding character ('a'-'h').
    /// A -> 'a', B -> 'b', ..., H -> 'h'
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessFile;
    /// assert_eq!(ChessFile::A.to_char(), 'a');
    /// assert_eq!(ChessFile::H.to_char(), 'h');
    /// ```
    pub fn to_char(&self) -> char {
        match self {
            ChessFile::A => 'a',
            ChessFile::B => 'b',
            ChessFile::C => 'c',
            ChessFile::D => 'd',
            ChessFile::E => 'e',
            ChessFile::F => 'f',
            ChessFile::G => 'g',
            ChessFile::H => 'h',
        }
    }

    /// Creates a ChessFile from a u8 value (1-8).
    /// Returns an error for values outside this range.
    pub fn from_u8(value: u8) -> std::result::Result<Self, ChessFileParseError> {
        match value {
            1 => Ok(ChessFile::A),
            2 => Ok(ChessFile::B),
            3 => Ok(ChessFile::C),
            4 => Ok(ChessFile::D),
            5 => Ok(ChessFile::E),
            6 => Ok(ChessFile::F),
            7 => Ok(ChessFile::G),
            8 => Ok(ChessFile::H),
            _ => Err(ChessFileParseError),
        }
    }

    /// Creates a ChessFile from a character ('a'-'h' or 'A'-'H').
    /// Returns an error for characters outside this range.
    pub fn from_char(c: char) -> std::result::Result<Self, ChessFileParseError> {
        match c {
            'a' | 'A' => Ok(ChessFile::A),
            'b' | 'B' => Ok(ChessFile::B),
            'c' | 'C' => Ok(ChessFile::C),
            'd' | 'D' => Ok(ChessFile::D),
            'e' | 'E' => Ok(ChessFile::E),
            'f' | 'F' => Ok(ChessFile::F),
            'g' | 'G' => Ok(ChessFile::G),
            'h' | 'H' => Ok(ChessFile::H),
            _ => Err(ChessFileParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {
        assert_eq!(ChessFile::A.to_int(), 1);
        assert_eq!(ChessFile::B.to_int(), 2);
        assert_eq!(ChessFile::C.to_int(), 3);
        assert_eq!(ChessFile::D.to_int(), 4);
        assert_eq!(ChessFile::E.to_int(), 5);
        assert_eq!(ChessFile::F.to_int(), 6);
        assert_eq!(ChessFile::G.to_int(), 7);
        assert_eq!(ChessFile::H.to_int(), 8);
    }

    #[test]
    fn test_to_u8() {
        assert_eq!(ChessFile::A.to_u8(), 1);
        assert_eq!(ChessFile::B.to_u8(), 2);
        assert_eq!(ChessFile::C.to_u8(), 3);
        assert_eq!(ChessFile::D.to_u8(), 4);
        assert_eq!(ChessFile::E.to_u8(), 5);
        assert_eq!(ChessFile::F.to_u8(), 6);
        assert_eq!(ChessFile::G.to_u8(), 7);
        assert_eq!(ChessFile::H.to_u8(), 8);
    }

    #[test]
    fn test_to_char() {
        assert_eq!(ChessFile::A.to_char(), 'a');
        assert_eq!(ChessFile::B.to_char(), 'b');
        assert_eq!(ChessFile::C.to_char(), 'c');
        assert_eq!(ChessFile::D.to_char(), 'd');
        assert_eq!(ChessFile::E.to_char(), 'e');
        assert_eq!(ChessFile::F.to_char(), 'f');
        assert_eq!(ChessFile::G.to_char(), 'g');
        assert_eq!(ChessFile::H.to_char(), 'h');
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(ChessFile::from_u8(1), Ok(ChessFile::A));
        assert_eq!(ChessFile::from_u8(2), Ok(ChessFile::B));
        assert_eq!(ChessFile::from_u8(3), Ok(ChessFile::C));
        assert_eq!(ChessFile::from_u8(4), Ok(ChessFile::D));
        assert_eq!(ChessFile::from_u8(5), Ok(ChessFile::E));
        assert_eq!(ChessFile::from_u8(6), Ok(ChessFile::F));
        assert_eq!(ChessFile::from_u8(7), Ok(ChessFile::G));
        assert_eq!(ChessFile::from_u8(8), Ok(ChessFile::H));
        assert_eq!(ChessFile::from_u8(9), Err(ChessFileParseError));
    }

    #[test]
    fn test_from_char() {
        assert_eq!(ChessFile::from_char('a'), Ok(ChessFile::A));
        assert_eq!(ChessFile::from_char('b'), Ok(ChessFile::B));
        assert_eq!(ChessFile::from_char('c'), Ok(ChessFile::C));
        assert_eq!(ChessFile::from_char('d'), Ok(ChessFile::D));
        assert_eq!(ChessFile::from_char('e'), Ok(ChessFile::E));
        assert_eq!(ChessFile::from_char('f'), Ok(ChessFile::F));
        assert_eq!(ChessFile::from_char('g'), Ok(ChessFile::G));
        assert_eq!(ChessFile::from_char('h'), Ok(ChessFile::H));
        assert_eq!(ChessFile::from_char('i'), Err(ChessFileParseError));
    }
}
