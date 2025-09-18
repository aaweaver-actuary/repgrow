use anyhow::Result;

use crate::{
    orchestration::{ExpansionInput, MoveApplierPort, SelectedCandidates},
    search::arena::NodeArenaStore,
};

pub struct NodeExpander<'a> {
    pub arena: &'a dyn NodeArenaStore,
    pub applier: &'a dyn MoveApplierPort,
}

impl<'a> NodeExpander<'a> {
    pub async fn expand(
        &self,
        parent: &ExpansionInput,
        selected: SelectedCandidates,
    ) -> Result<Vec<u64>> {
        // loop selected.moves:
        //   applier.apply(input.fen_key, uci) → next_fen
        //   arena.push(child) + arena.push_child(parent.id, child_id)
        // collect child_ids
    }
}
// Unit test: stub MoveApplierPort to return deterministic FENs; assert children created & linked.
