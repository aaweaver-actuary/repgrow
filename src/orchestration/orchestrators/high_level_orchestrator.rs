use anyhow::Result;

use crate::{
    config::SearchConfig,
    domain::CandidateRequest,
    orchestration::{
        ChildEnqueuer, NodeExpansionOrchestrator, TerminationPolicyPort, WorkQueuePort,
    },
};

pub struct HighLevelOrchestrator<'a> {
    pub queue: &'a dyn WorkQueuePort,
    pub node_orch: NodeExpansionOrchestrator<'a>,
    pub enqueuer: ChildEnqueuer<'a>,
    pub termination: &'a dyn TerminationPolicyPort,
    pub cfg: &'a SearchConfig,
}

impl<'a> HighLevelOrchestrator<'a> {
    pub async fn run(&self, root_id: u64, max_plies: u32) -> Result<()> {
        self.queue.send(root_id).await?;
        let base_req = CandidateRequest {
            fen_key: todo!(),
            max_candidates: todo!(),
            cp_window: todo!(),
            min_play_rate: todo!(),
            multipv: todo!(),
        };

        // single-consumer dispatcher: read → spawn → track
        let mut joinset = tokio::task::JoinSet::new();

        while let Some(nid) = self.queue.recv().await {
            let node_orch = self.node_orch.clone_for_spawn(); // or Arc wrap
            let enq = self.enqueuer; // trait obj reference
            let base_req = base_req.clone();

            joinset.spawn(async move {
                if let Ok(children) = node_orch.expand_one(nid, &base_req).await {
                    let _ = enq.enqueue_all(&children).await;
                }
            });
        }

        // when sender is dropped (outside), drain remaining workers
        while let Some(_res) = joinset.join_next().await {}
        Ok(())
    }
}

// Integration test (no HTTP): dummy providers + in-mem queue + small caps → ensure build finishes and child count matches expectation.
