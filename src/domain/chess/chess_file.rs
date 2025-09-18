#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Returns None for values outside this range.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessFile;
    /// assert_eq!(ChessFile::from_u8(1), Some(ChessFile::A));
    /// assert_eq!(ChessFile::from_u8(8), Some(ChessFile::H));
    /// assert_eq!(ChessFile::from_u8(9), None);
    /// assert_eq!(ChessFile::from_u8(0), None);
    /// ```
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(ChessFile::A),
            2 => Some(ChessFile::B),
            3 => Some(ChessFile::C),
            4 => Some(ChessFile::D),
            5 => Some(ChessFile::E),
            6 => Some(ChessFile::F),
            7 => Some(ChessFile::G),
            8 => Some(ChessFile::H),
            _ => None,
        }
    }

    /// Creates a ChessFile from a character ('a'-'h' or 'A'-'H').
    /// Returns None for characters outside this range.
    /// # Examples
    /// ```
    /// use repgrow::domain::chess::ChessFile;
    /// assert_eq!(ChessFile::from_char('a'), Some(ChessFile::A));
    /// assert_eq!(ChessFile::from_char('H'), Some(ChessFile::H));
    /// assert_eq!(ChessFile::from_char('i'), None);
    /// assert_eq!(ChessFile::from_char('1'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' | 'A' => Some(ChessFile::A),
            'b' | 'B' => Some(ChessFile::B),
            'c' | 'C' => Some(ChessFile::C),
            'd' | 'D' => Some(ChessFile::D),
            'e' | 'E' => Some(ChessFile::E),
            'f' | 'F' => Some(ChessFile::F),
            'g' | 'G' => Some(ChessFile::G),
            'h' | 'H' => Some(ChessFile::H),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(ChessFile::from_u8(1), Some(ChessFile::A));
        assert_eq!(ChessFile::from_u8(2), Some(ChessFile::B));
        assert_eq!(ChessFile::from_u8(3), Some(ChessFile::C));
        assert_eq!(ChessFile::from_u8(4), Some(ChessFile::D));
        assert_eq!(ChessFile::from_u8(5), Some(ChessFile::E));
        assert_eq!(ChessFile::from_u8(6), Some(ChessFile::F));
        assert_eq!(ChessFile::from_u8(7), Some(ChessFile::G));
        assert_eq!(ChessFile::from_u8(8), Some(ChessFile::H));
        assert_eq!(ChessFile::from_u8(9), None);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(ChessFile::from_char('a'), Some(ChessFile::A));
        assert_eq!(ChessFile::from_char('b'), Some(ChessFile::B));
        assert_eq!(ChessFile::from_char('c'), Some(ChessFile::C));
        assert_eq!(ChessFile::from_char('d'), Some(ChessFile::D));
        assert_eq!(ChessFile::from_char('e'), Some(ChessFile::E));
        assert_eq!(ChessFile::from_char('f'), Some(ChessFile::F));
        assert_eq!(ChessFile::from_char('g'), Some(ChessFile::G));
        assert_eq!(ChessFile::from_char('h'), Some(ChessFile::H));
        assert_eq!(ChessFile::from_char('i'), None);
    }
}
