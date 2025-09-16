use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn to_string(&self) -> &str {
        match self {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' | 'W' => Some(PieceColor::White),
            'b' | 'B' => Some(PieceColor::Black),
            _ => None,
        }
    }

    pub fn to_shakmaty(&self) -> shakmaty::Color {
        match self {
            PieceColor::White => shakmaty::Color::White,
            PieceColor::Black => shakmaty::Color::Black,
        }
    }

    pub fn from_shakmaty(c: shakmaty::Color) -> Self {
        match c {
            shakmaty::Color::White => PieceColor::White,
            shakmaty::Color::Black => PieceColor::Black,
        }
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
