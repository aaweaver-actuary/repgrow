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
use tracing::{debug, info};

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
        info!(
            "Orchestrator: build_from_start called with san_line={:?}, max_plies={}",
            san_line, max_plies
        );
        let (root_fen, _stm) = start_from_san(san_line)?;
        debug!("Root FEN: {:?}", root_fen);
        let root_id = self.arena.push(make_node(None, &root_fen, None, 0)).await;
        debug!("Root node pushed with id: {}", root_id);
        let root = self.arena.get(root_id).await.expect("root in arena");

        let (tx, mut rx) = mpsc::channel::<u64>(self.cfg.concurrency * 4);
        tx.send(root.id).await.ok();

        let mut joinset = JoinSet::new();

        while let Some(nid) = rx.recv().await {
            debug!("Dequeued node id: {} for expansion", nid);
            let tx2 = tx.clone();
            let cfg2 = self.cfg.clone();
            let policy2 = Arc::clone(&self.policy);
            let quality2 = Arc::clone(&self.quality);
            let popularity2 = Arc::clone(&self.popularity);
            let seen2 = self.seen.clone();
            let arena_ref = self.arena.clone();

            joinset.spawn(async move {
                debug!("Worker spawned for node id: {}", nid);
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
                debug!("Worker finished for node id: {}", nid);
            });
        }

        // Sender dropped → wait for all in-flight workers to finish
        info!("Waiting for all workers to finish...");
        while let Some(_res) = joinset.join_next().await {}
        info!("All workers finished. Returning root node.");

        Ok(root)
    }
    /// Returns a clone of all nodes in the arena (for testing/inspection).
    pub async fn all_nodes(&self) -> Vec<crate::domain::RepertoireNode> {
        self.arena.all_nodes().await
    }
}
