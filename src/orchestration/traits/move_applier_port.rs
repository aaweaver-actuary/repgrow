use anyhow::Result;

use crate::domain::{FenKey, chess::UciMove};

pub trait MoveApplierPort {
    fn apply(&self, fen: &FenKey, uci: UciMove) -> Result<FenKey>;
}
