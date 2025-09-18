use crate::domain::chess::{ChessPieceType, ChessSquare};

pub struct Uci {
    pub start_square: ChessSquare,
    pub end_square: ChessSquare,
    pub promotion: Option<ChessPieceType>,
    pub is_en_passant: Option<bool>,
    pub is_castling: Option<bool>,
}
