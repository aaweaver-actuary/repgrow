use crate::domain::{FenKey, RepertoireNode};

/// Immutable snapshot required to expand a node.
pub struct ExpansionInput {
    pub node_id: u64,
    pub fen_key: FenKey,
    pub ply_depth: u32,
}

impl ExpansionInput {
    pub fn new(node: &RepertoireNode) -> Self {
        Self {
            node_id: node.id,
            fen_key: node.fen_key.clone(),
            ply_depth: node.ply_depth,
        }
    }
}
