use anyhow::Result;

use crate::{
    domain::CandidateRequest,
    orchestration::{CandidateSelector, ExpansionPlanner, NodeExpander, ProviderDecisionEngine},
};

pub struct NodeExpansionOrchestrator<'a> {
    pub planner: ExpansionPlanner<'a>,
    pub provider: ProviderDecisionEngine<'a>,
    pub selector: CandidateSelector<'a>,
    pub expander: NodeExpander<'a>,
}

impl<'a> NodeExpansionOrchestrator<'a> {
    pub async fn expand_one(&self, node_id: u64, base_req: &CandidateRequest) -> Result<Vec<u64>> {
        // plan
        let Some(input) = self.planner.plan(node_id).await? else {
            return Ok(vec![]);
        };
        // fetch
        let raw = self.provider.fetch_raw(&input, base_req).await?;
        // select
        let selected = self.selector.select(&input, raw);
        // expand
        let child_ids = self.expander.expand(&input, selected).await?;
        Ok(child_ids)
    }
}

// Unit test: mock planner→provider→selector→expander; assert composition order & outputs.
