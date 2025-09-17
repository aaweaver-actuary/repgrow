use super::{
    arena::{MemArena, NodeArenaStore},
    build::{make_node, start_from_san},
    worker::expand_node_task,
};
use crate::{
    config::SearchConfig,
    domain::{FenKey, RepertoireNode},
    policy::MovePolicy,
    provider::{MovePopularity, MoveQuality},
};
use dashmap::DashSet;
use std::sync::Arc;
use tokio::{sync::mpsc, task::JoinSet};

/// Orchestrator: drains the work queue (single consumer) and spawns a worker per item.
pub struct Orchestrator {
    cfg: SearchConfig,
    policy: Arc<dyn MovePolicy>,
    quality: Arc<dyn MoveQuality>,
    popularity: Arc<dyn MovePopularity>,
    arena: MemArena,
    seen: DashSet<FenKey>,
}

impl Orchestrator {
    pub fn new(
        cfg: SearchConfig,
        policy: impl MovePolicy + 'static,
        quality: Arc<dyn MoveQuality>,
        popularity: Arc<dyn MovePopularity>,
    ) -> Self {
        Self {
            cfg,
            policy: Arc::new(policy),
            quality,
            popularity,
            arena: MemArena::new(),
            seen: DashSet::new(),
        }
    }

    /// Build repertoire from an optional SAN line and expand up to `max_plies`.
    /// Single-consumer dispatcher pattern: no Receiver clones.
    pub async fn build_from_start(
        &self,
        san_line: Option<&str>,
        max_plies: u32,
    ) -> anyhow::Result<RepertoireNode> {
        let (root_fen, _stm) = start_from_san(san_line)?;
        let root_id = self.arena.push(make_node(None, &root_fen, None, 0)).await;
        let root = self.arena.get(root_id).await.expect("root in arena");

        let (tx, mut rx) = mpsc::channel::<u64>(self.cfg.concurrency * 4);
        tx.send(root.id).await.ok();

        let mut joinset = JoinSet::new();

        while let Some(nid) = rx.recv().await {
            let tx2 = tx.clone();
            let cfg2 = self.cfg.clone();
            let policy2 = Arc::clone(&self.policy);
            let quality2 = Arc::clone(&self.quality);
            let popularity2 = Arc::clone(&self.popularity);
            let seen2 = self.seen.clone();
            let arena_ref = self.arena.clone();

            joinset.spawn(async move {
                let _ = expand_node_task(
                    nid,
                    max_plies,
                    &cfg2,
                    &*policy2,
                    &quality2,
                    &popularity2,
                    &arena_ref,
                    &seen2,
                    &tx2,
                )
                .await;
            });
        }

        // Sender dropped → wait for all in-flight workers to finish
        while let Some(_res) = joinset.join_next().await {}

        Ok(root)
    }
    /// Returns a clone of all nodes in the arena (for testing/inspection).
    pub async fn all_nodes(&self) -> Vec<crate::domain::RepertoireNode> {
        self.arena.all_nodes().await
    }
}
