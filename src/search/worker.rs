use super::{arena::NodeArenaStore, util::apply_uci};
use crate::{
    config::SearchConfig,
    domain::{CandidateRequest, FenKey, RepertoireNode},
    policy::{Decision, MovePolicy},
    provider::{normalize_popularity, normalize_quality, MovePopularity, MoveQuality},
};
use dashmap::DashSet;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn expand_node_task(
    nid: u64,
    max_plies: u32,
    cfg: &SearchConfig,
    policy: &dyn MovePolicy,
    quality: &Arc<dyn MoveQuality>,
    popularity: &Arc<dyn MovePopularity>,
    arena: &dyn NodeArenaStore,
    seen: &DashSet<FenKey>,
    tx: &mpsc::Sender<u64>,
) -> anyhow::Result<()> {
    // Snapshot minimal node data
    let (fen_key, ply_depth) = {
        let n = arena
            .get(nid)
            .await
            .ok_or_else(|| anyhow::anyhow!("missing node {nid}"))?;
        (n.fen_key.clone(), n.ply_depth)
    };

    if ply_depth >= max_plies {
        return Ok(());
    }
    if !seen.insert(fen_key.clone()) {
        return Ok(());
    }

    let mut req = CandidateRequest {
        fen_key: fen_key.clone(),
        max_candidates: 8,
        cp_window: 50.0,
        min_play_rate: 0.07,
        multipv: 8,
    };

    let is_my_side = matches!(policy.decide(fen_key.side_to_move.to_shakmaty()), Decision::Quality);
    policy.adjust(&mut req, is_my_side);

    let mut cands = match policy.decide(fen_key.side_to_move.to_shakmaty()) {
        Decision::Quality => {
            let evals = quality.evaluate(&req.fen_key, Some(req.multipv)).await?;
            normalize_quality(&req.fen_key, evals)
        }
        Decision::Popularity => {
            let rows = popularity.sample(&req.fen_key).await?;
            normalize_popularity(&req.fen_key, rows)
        }
        Decision::Hybrid => Vec::new(),
    };

    // Post-filter + cap
    cands = policy.post_filter(cands);
    let cap = if is_my_side {
        cfg.max_children_my_side
    } else {
        cfg.max_children_opp_side
    };
    cands.truncate(cap.expect("max_children should be set"));

    // Apply moves → create children → enqueue
    let mut child_ids = Vec::with_capacity(cands.len());
    for c in cands {
        if let Ok((next_fen, _stm)) = apply_uci(&fen_key, &c.uci) {
            let child = RepertoireNode {
                id: 0,
                parent: Some(nid),
                fen_key: next_fen,
                last_move_uci: Some(c.uci.clone()),
                ply_depth: ply_depth + 1,
                children: Vec::new(),
                signals: Default::default(),
            };
            let child_id = arena.push(child).await;
            arena.push_child(nid, child_id).await;
            child_ids.push(child_id);
        }
    }

    for cid in child_ids {
        tx.send(cid).await.ok();
    }
    Ok(())
}
