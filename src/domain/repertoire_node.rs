use crate::domain::Signals;

use super::fen_key::FenKey;

#[derive(Clone, Debug)]
pub struct RepertoireNode {
    pub id: u64,
    pub parent: Option<u64>,
    pub fen_key: FenKey,
    pub last_move_uci: Option<String>,
    pub ply_depth: u32,
    pub children: Vec<u64>,
    pub signals: Signals,
}
