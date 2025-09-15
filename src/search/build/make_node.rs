use crate::domain::{FenKey, RepertoireNode};

/// Create a node struct (id assigned by arena)
pub fn make_node(
    parent: Option<u64>,
    fen_key: &FenKey,
    last_uci: Option<String>,
    ply: u32,
) -> RepertoireNode {
    RepertoireNode {
        id: 0, // arena will set real id
        parent,
        fen_key: fen_key.clone(),
        last_move_uci: last_uci,
        ply_depth: ply,
        children: Vec::new(),
        signals: Default::default(),
    }
}
