use anyhow::Result;
use dashmap::DashSet;

use crate::{
    domain::FenKey,
    orchestration::{ExpansionInput, TerminationPolicyPort},
    search::arena::NodeArenaStore,
};

pub struct ExpansionPlanner<'a> {
    pub arena: &'a dyn NodeArenaStore,
    pub seen: &'a DashSet<FenKey>,
    pub termination: &'a dyn TerminationPolicyPort,
}

impl<'a> ExpansionPlanner<'a> {
    pub async fn plan(&self, node_id: u64) -> Result<Option<ExpansionInput>> {
        // 1) read node (snapshot fen, ply)
        // 2) termination.check(ply) → early return None
        // 3) seen.insert(fen) → skip duplicates (return None)
        // 4) return ExpansionInput if work should proceed
    }
}

// Unit test: mock arena with known node → expect Some(ExpansionInput). test termination and dedup paths return None.
