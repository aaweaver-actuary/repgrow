use crate::{
    config::SearchCfg,
    domain::{CandidateRequest, FenKey, PieceColor, RepertoireNode},
    infra::Infra,
    policy::{Decision, MovePolicy},
    provider::{normalize_popularity, normalize_quality, MovePopularity, MoveQuality},
};
use dashmap::DashSet;
use shakmaty::{
    fen::Fen, san::San, uci::Uci, CastlingMode::Standard, Chess, EnPassantMode::Legal, Position,
};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct Orchestrator {
    cfg: SearchCfg,
    policy: Arc<dyn MovePolicy>,
    quality: Arc<dyn MoveQuality>,
    popularity: Arc<dyn MovePopularity>,
    infra: Infra,

    // <-- own the mutable arena behind an async-friendly lock
    nodes: Arc<Mutex<Vec<RepertoireNode>>>,
    seen: DashSet<FenKey>,
}

impl Orchestrator {
    pub fn new(
        cfg: SearchCfg,
        policy: impl MovePolicy + 'static,
        q: Arc<dyn MoveQuality>,
        p: Arc<dyn MovePopularity>,
        infra: Infra,
    ) -> Self {
        Self {
            cfg,
            policy: Arc::new(policy),
            quality: q,
            popularity: p,
            infra,
            nodes: Arc::new(Mutex::new(Vec::new())),
            seen: DashSet::new(),
        }
    }

    pub async fn build_from_start(
        &self,
        san_line: Option<&str>,
        max_plies: u32,
    ) -> anyhow::Result<RepertoireNode> {
        let (root_fen, _stm) = self.start_from_san(san_line)?;
        let root = self.push_node(None, &root_fen, None, 0).await?;

        let (tx, mut rx) = mpsc::channel::<u64>(self.cfg.concurrency * 4);
        tx.send(root.id).await.ok();

        // Clone all the Arcs/config needed by workers (no &mut self captured)
        let nodes = Arc::clone(&self.nodes);
        let seen = &self.seen;
        let policy = Arc::clone(&self.policy);
        let quality = Arc::clone(&self.quality);
        let popularity = Arc::clone(&self.popularity);
        let cfg = self.cfg.clone();

        // Spawn bounded worker pool
        for _ in 0..self.cfg.concurrency {
            let rx2 = rx.clone();
            let tx2 = tx.clone();
            let nodes2 = Arc::clone(&nodes);
            let policy2 = Arc::clone(&policy);
            let quality2 = Arc::clone(&quality);
            let popularity2 = Arc::clone(&popularity);
            let seen2 = seen; // DashSet is Sync
            let cfg2 = cfg.clone();

            tokio::spawn(async move {
                let mut rx = rx2;
                while let Some(nid) = rx.recv().await {
                    // Expand node; on success, enqueue children
                    if let Err(_e) = expand_node_task(
                        nid,
                        max_plies,
                        &cfg2,
                        &policy2,
                        &quality2,
                        &popularity2,
                        &nodes2,
                        seen2,
                        &tx2,
                    )
                    .await
                    {
                        // You may want to log `_e` with tracing
                    }
                }
            });
        }

        drop(tx); // allow workers to exit when the queue drains

        // Wait until the receiver side is fully dropped (all workers exited)
        while rx.recv().await.is_some() {}

        Ok(root)
    }

    fn start_from_san(&self, san_line: Option<&str>) -> anyhow::Result<(FenKey, shakmaty::Color)> {
        let mut pos = Chess::default();
        if let Some(line) = san_line {
            for tok in line.split_whitespace() {
                if tok.contains('.') {
                    continue;
                }
                let san: San = tok.parse().map_err(|_| anyhow::anyhow!("bad SAN: {tok}"))?;
                let mv = san
                    .to_move(&pos)
                    .map_err(|_| anyhow::anyhow!("illegal SAN: {tok}"))?;
                pos.play_unchecked(&mv);
            }
        }
        let fen = Fen::from_position(pos.clone(), Legal).to_string();
        let stm = pos.turn();
        Ok((
            FenKey {
                fen,
                stm: PieceColor::from_shakmaty(stm),
            },
            stm,
        ))
    }

    async fn push_node(
        &self,
        parent: Option<u64>,
        fen_key: &FenKey,
        last_uci: Option<String>,
        ply: u32,
    ) -> anyhow::Result<RepertoireNode> {
        let mut nodes = self.nodes.lock().await;
        let id = nodes.len() as u64;
        let node = RepertoireNode {
            id,
            parent,
            fen_key: fen_key.clone(),
            last_move_uci: last_uci,
            ply_depth: ply,
            children: Vec::new(),
            signals: Default::default(),
        };
        nodes.push(node.clone());
        Ok(node)
    }
}

// ------------- worker function (no &mut self captured) ----------------

async fn expand_node_task(
    nid: u64,
    max_plies: u32,
    cfg: &SearchCfg,
    policy: &dyn MovePolicy,
    quality: &Arc<dyn MoveQuality>,
    popularity: &Arc<dyn MovePopularity>,
    nodes: &Arc<Mutex<Vec<RepertoireNode>>>,
    seen: &DashSet<FenKey>,
    tx: &mpsc::Sender<u64>,
) -> anyhow::Result<()> {
    // Snapshot what we need from the node (avoid holding the lock too long)
    let (fen_key, ply_depth) = {
        let nodes_guard = nodes.lock().await;
        let n = &nodes_guard[nid as usize];
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
        cp_window: 50, // i32, not f32
        min_play_rate: 0.07,
        multipv: 8,
    };

    let is_my_side = matches!(policy.decide(fen_key.stm.to_shakmaty()), Decision::Quality);
    policy.adjust(&mut req, is_my_side);

    // Fetch candidates (no arena mutation yet)
    let mut cands = match policy.decide(fen_key.stm.to_shakmaty()) {
        Decision::Quality => {
            let evals = quality.evaluate(&req.fen_key, req.multipv).await?;
            normalize_quality(&req.fen_key, evals)
        }
        Decision::Popularity => {
            let rows = popularity.sample(&req.fen_key).await?;
            normalize_popularity(&req.fen_key, rows)
        }
        Decision::Hybrid => {
            // (future) shortlist via popularity then filter via quality
            Vec::new()
        }
    };

    // Post-filter + cap by side
    cands = policy.post_filter(is_my_side, cands);
    let cap = if is_my_side {
        cfg.max_children_my_side
    } else {
        cfg.max_children_opp_side
    };
    cands.truncate(cap);

    // Build children by applying UCI → next FEN, then mutate arena atomically
    let mut new_child_ids = Vec::with_capacity(cands.len());
    for c in cands {
        if let Ok((next_fen, _stm)) = apply_uci(&fen_key, &c.uci) {
            // push_node equivalent under the lock
            let mut nodes_guard = nodes.lock().await;
            let new_id = nodes_guard.len() as u64;
            let child = RepertoireNode {
                id: new_id,
                parent: Some(nid),
                fen_key: next_fen,
                last_move_uci: Some(c.uci.clone()),
                ply_depth: ply_depth + 1,
                children: Vec::new(),
                signals: Default::default(),
            };
            nodes_guard.push(child);
            nodes_guard[nid as usize].children.push(new_id);
            new_child_ids.push(new_id);
        }
    }

    // Enqueue children for further expansion
    for cid in new_child_ids {
        tx.send(cid).await.ok();
    }

    Ok(())
}

// ------------- small helpers (pure) ----------------

fn apply_uci(fen_key: &FenKey, uci: &str) -> anyhow::Result<(FenKey, shakmaty::Color)> {
    let pos: Chess = fen_key
        .fen
        .parse::<shakmaty::fen::Fen>()?
        .into_position(Standard)?;
    let u: Uci = uci.parse()?;
    let m: shakmaty::Move = u
        .to_move(&pos)
        .map_err(|_| anyhow::anyhow!("illegal UCI"))?;
    let mut next = pos.clone();
    next.play_unchecked(&m);
    let next_fen = shakmaty::fen::Fen::from_position(next.clone(), Legal).to_string();
    Ok((
        FenKey {
            fen: next_fen,
            stm: PieceColor::from_shakmaty(next.turn()),
        },
        next.turn(),
    ))
}
