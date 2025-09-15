use serde::{Deserialize, Serialize};

use crate::domain::PieceColor;

/// FEN + side-to-move forms a canonical lookup key.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FenKey {
    pub fen: String,
    pub stm: PieceColor,
}
