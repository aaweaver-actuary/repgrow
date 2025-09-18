use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    /// Returns true if the color is white.
    pub fn is_white(&self) -> bool {
        matches!(self, PieceColor::White)
    }

    /// Returns true if the color is black.
    pub fn is_black(&self) -> bool {
        matches!(self, PieceColor::Black)
    }
    /// Returns "white" or "black".
    /// # Examples
    /// ```
    /// use repgrow::domain::PieceColor;
    /// let white = PieceColor::White;
    /// let black = PieceColor::Black;
    /// assert_eq!(white.to_string(), "white");
    /// assert_eq!(black.to_string(), "black");
    /// ```
    pub fn to_string(&self) -> &str {
        match self {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }

    /// Creates a PieceColor from a character ('w' or 'b', case insensitive).
    /// Returns None for invalid characters.
    /// # Examples
    /// ```
    /// use repgrow::domain::PieceColor;
    /// assert_eq!(PieceColor::from_char('w'), Some(PieceColor::White));
    /// assert_eq!(PieceColor::from_char('b'), Some(PieceColor::Black));
    /// assert_eq!(PieceColor::from_char('W'), Some(PieceColor::White));
    /// assert_eq!(PieceColor::from_char('B'), Some(PieceColor::Black));
    /// assert_eq!(PieceColor::from_char('x'), None);
    /// ```
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' | 'W' => Some(PieceColor::White),
            'b' | 'B' => Some(PieceColor::Black),
            _ => None,
        }
    }

    /// Converts to shakmaty::Color. Sometimes my PieceColor is easier, sometimes shakmaty's is.
    /// # Examples
    /// ```
    /// use repgrow::domain::PieceColor;
    /// use shakmaty::Color;
    /// let white = PieceColor::White;
    /// let black = PieceColor::Black;
    /// assert_eq!(white.to_shakmaty(), Color::White);
    /// assert_eq!(black.to_shakmaty(), Color::Black);
    /// ```
    pub fn to_shakmaty(&self) -> shakmaty::Color {
        match self {
            PieceColor::White => shakmaty::Color::White,
            PieceColor::Black => shakmaty::Color::Black,
        }
    }

    /// Converts from shakmaty::Color to PieceColor.
    /// # Examples
    /// ```
    /// use repgrow::domain::PieceColor;
    /// use shakmaty::Color;
    /// assert_eq!(PieceColor::from_shakmaty(Color::White), PieceColor::White);
    /// assert_eq!(PieceColor::from_shakmaty(Color::Black), PieceColor::Black);
    /// ```
    pub fn from_shakmaty(c: shakmaty::Color) -> Self {
        match c {
            shakmaty::Color::White => PieceColor::White,
            shakmaty::Color::Black => PieceColor::Black,
        }
    }
}

impl Display for PieceColor {
    /// Formats the PieceColor as "white" or "black".
    /// # Examples
    /// ```
    /// use repgrow::domain::PieceColor;
    /// let white = PieceColor::White;
    /// let black = PieceColor::Black;
    /// assert_eq!(format!("{}", white), "white");
    /// assert_eq!(format!("{}", black), "black");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
