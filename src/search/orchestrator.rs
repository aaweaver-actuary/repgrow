use crate::{
    config::SearchConfig,
    domain::{CandidateRequest, FenKey, PieceColor, RepertoireNode},
    infra::Infra,
    policy::{Decision, MovePolicy},
    provider::{normalize_popularity, normalize_quality, MovePopularity, MoveQuality},
};
use anyhow::{anyhow, Result};
use dashmap::DashSet;
use shakmaty::{
    fen::Fen, san::San, uci::Uci, CastlingMode::Standard, Chess, Color, EnPassantMode::Legal,
    Position,
};
use std::sync::Arc;
use tokio::{
    spawn,
    sync::{mpsc, Mutex},
};

type SafeRepertoireNodeList = Arc<Mutex<Vec<RepertoireNode>>>;
type MoveProviderSelectionPolicy = Arc<dyn MovePolicy>;

/// Orchestrator: build repertoire tree from a starting position (FEN or SAN line).
/// Uses a bounded pool of workers to expand nodes concurrently.
/// The mutable arena is owned here behind an async-friendly lock.
/// The seen set is a concurrent DashSet to avoid duplicate positions.
pub struct Orchestrator {
    /// Immutable config + providers + infra
    cfg: Option<SearchConfig>,

    /// `policy` decides which provider to use per position.
    /// Policy used to decide which move provider to use per position.
    /// This is an Arc-wrapped trait object for thread-safe shared ownership.
    policy: MoveProviderSelectionPolicy,

    /// `quality` provider (e.g. Lichess cloud eval).
    quality: Arc<dyn MoveQuality>,

    /// `popularity` provider (e.g. Lichess openings explorer).
    popularity: Arc<dyn MovePopularity>,

    /// Infra (e.g. HTTP client, rate limiters, cache).
    infra: Infra,

    /// Mutable repertoire nodes (arena).
    nodes: SafeRepertoireNodeList,

    /// Concurrent set of seen FENs to avoid hitting the API multiple times.
    /// Expecting many lines will share positions, so this is important.
    /// DashSet is Sync and can be shared across workers.
    seen: DashSet<FenKey>,
}

impl Orchestrator {
    /// Create a new orchestrator with the given config, policy, providers, and infra.
    /// The mutable arena and seen set are initialized empty.
    /// The providers should be built from the same infra to share HTTP, cache, rate limit
    pub fn new(
        cfg: Option<SearchConfig>,
        policy: MoveProviderSelectionPolicy,
        quality: Arc<dyn MoveQuality>,
        popularity: Arc<dyn MovePopularity>,
        infra: Infra,
    ) -> Self {
        let c = cfg.unwrap_or_default();
        Self {
            cfg: Some(c),
            policy,
            quality,
            popularity,
            infra,
            nodes: Arc::new(Mutex::new(Vec::new())),
            seen: DashSet::new(),
        }
    }

    /// Return the configured concurrency level.
    pub fn concurrency(&self) -> usize {
        self.cfg.as_ref().map_or(4, |c| c.concurrency)
    }

    /// Return the configured max total nodes, or 1000000000 if not set.
    pub fn max_total_nodes(&self) -> usize {
        self.cfg
            .as_ref()
            .and_then(|c| c.max_total_nodes)
            .unwrap_or(1000000000)
    }

    /// Return the configured max children for my side, or 10000 if not set.
    pub fn max_children_my_side(&self) -> usize {
        self.cfg
            .as_ref()
            .and_then(|c| c.max_children_my_side)
            .unwrap_or(10000)
    }

    /// Return the configured max children for opponent's side, or 10000 if not set.
    pub fn max_children_opp_side(&self) -> usize {
        self.cfg
            .as_ref()
            .and_then(|c| c.max_children_opp_side)
            .unwrap_or(10000)
    }

    /// Build the repertoire tree from the given SAN line (or startpos if None).
    /// Expands nodes up to max_plies using a bounded pool of workers.
    /// Returns the root node of the built tree.
    /// The tree is stored in an arena (Vec) inside a Mutex for async safety.
    /// Each worker fetches candidates from the appropriate provider based on policy.
    /// The seen set avoids duplicate positions across lines.
    /// The max_plies limits the depth of the tree.
    /// The concurrency config controls the number of worker tasks.
    /// This function is async and should be awaited.
    pub async fn build_from_start(
        &self,
        san_line: Option<&str>,
        max_plies: u32,
    ) -> Result<RepertoireNode> {
        let (root_fen, _stm) = self.start_from_san(san_line)?;
        let root_repertoire_node = self.push_node(None, &root_fen, None, 0).await?;

        let max_message_buffer_size = self.concurrency() * 4;
        let (sender, mut receiver) = mpsc::channel::<u64>(max_message_buffer_size);

        sender.send(root_repertoire_node.id).await.ok();

        // Clone all the Arcs/config needed by workers (no &mut self captured)
        let nodes = Arc::clone(&self.nodes);
        let seen = &self.seen;
        let policy = Arc::clone(&self.policy);
        let quality = Arc::clone(&self.quality);
        let popularity = Arc::clone(&self.popularity);
        let cfg = self.cfg.clone();

        // Spawn bounded worker pool
        for _ in 0..self.concurrency() {
            let receiver2 = receiver;
            let sender2 = sender.clone();
            let nodes2 = Arc::clone(&nodes);
            let policy2 = Arc::clone(&policy);
            let quality2 = Arc::clone(&quality);
            let popularity2 = Arc::clone(&popularity);
            let seen2 = seen; // DashSet is Sync
            let cfg2 = cfg.clone();

            spawn(async move {
                let mut receiver3 = receiver2;
                while let Some(nid) = receiver3.recv().await {
                    let cfg_copy = cfg2.expect("cfg should be set").clone();

                    // Expand node; on success, enqueue children
                    match expand_node_task(
                        nid,
                        max_plies,
                        &cfg_copy,
                        &policy2,
                        &quality2,
                        &popularity2,
                        &nodes2,
                        seen2,
                        &sender2,
                    )
                    .await
                    {
                        Err(_e) => {}
                        Ok(_) => todo!(),
                    }
                }
            });
        }

        drop(sender); // allow workers to exit when the queue drains

        // Wait until the receiver side is fully dropped (all workers exited)
        while receiver.recv().await.is_some() {}

        Ok(root_repertoire_node)
    }

    fn start_from_san(&self, san_line: Option<&str>) -> Result<(FenKey, Color)> {
        let mut position = Chess::default();
        if let Some(line) = san_line {
            // Loop over tokens, ignoring move numbers (e.g. "1.", "2.", etc)
            for token in line.split_whitespace() {
                if token.contains('.') {
                    continue;
                }

                // Parse SAN into a SAN struct
                // let san: San = token.parse().map_err(|_| anyhow!("bad SAN: {token}"))?;
                let san_move = token
                    .parse::<San>()
                    .map_err(|_| anyhow!("bad SAN: {token}"))?
                    .to_move(&position)
                    .map_err(|_| anyhow!("illegal SAN: {token}"))?;

                // Apply the move to the position (we can play_unchecked since it
                // was validated in the previous step)
                position.play_unchecked(&san_move);
            }
        }
        let fen = Fen::from_position(position.clone(), Legal).to_string();
        let stm = position.turn();
        Ok((
            FenKey {
                fen_string: fen,
                side_to_move: PieceColor::from_shakmaty(stm),
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
    ) -> Result<RepertoireNode> {
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

/// Given a locked set of nodes, expand the given node ID and return the FEN string.
async fn expand_node_fen(nodes: &SafeRepertoireNodeList, node_id: u64) -> Option<FenKey> {
    let nodes_guard = nodes.lock().await;
    let node = &nodes_guard[node_id as usize];
    Some(node.fen_key.clone())
}

/// Given a node ID, return its ply depth.
async fn expand_node_ply(nodes: &SafeRepertoireNodeList, node_id: u64) -> Option<u32> {
    let nodes_guard = nodes.lock().await;
    let node = &nodes_guard[node_id as usize];
    Some(node.ply_depth)
}

/// Given a node ID, return its fen key and ply depth.
/// Returns None if the node ID is invalid.
async fn expand_node_snapshot(nodes: &SafeRepertoireNodeList, nid: u64) -> Option<(FenKey, u32)> {
    let nodes_guard = nodes.lock().await;
    let n = nodes_guard.get(nid as usize)?;
    Some((n.fen_key.clone(), n.ply_depth))
}

async fn expand_node_task(
    node_id: u64,
    max_plies: u32,
    cfg: &SearchConfig,
    policy: &MoveProviderSelectionPolicy,
    quality: &Arc<dyn MoveQuality>,
    popularity: &Arc<dyn MovePopularity>,
    nodes: &SafeRepertoireNodeList,
    seen: &DashSet<FenKey>,
    sender: &mpsc::Sender<u64>,
) -> Result<()> {
    // Snapshot what we need from the node (avoid holding the lock too long)
    let fen_key = expand_node_fen(nodes, node_id)
        .await
        .ok_or_else(|| anyhow!("missing node {node_id}"))?;

    let ply_depth = expand_node_ply(nodes, node_id)
        .await
        .ok_or_else(|| anyhow!("missing node {node_id}"))?;

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

    let is_my_side = matches!(
        policy.decide(fen_key.side_to_move.to_shakmaty()),
        Decision::Quality
    );
    policy.adjust(&mut req, is_my_side);

    // Fetch candidates (no arena mutation yet)
    let mut cands = match policy.decide(fen_key.side_to_move.to_shakmaty()) {
        Decision::Quality => {
            let evals = quality.evaluate(&req.fen_key, Some(req.multipv)).await?;
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
    cands = policy.post_filter(cands);
    let cap = if is_my_side {
        cfg.max_children_my_side
    } else {
        cfg.max_children_opp_side
    };
    cands.truncate(cap.expect("max_children should be set"));

    // Build children by applying UCI → next FEN, then mutate arena atomically
    let mut new_child_ids = Vec::with_capacity(cands.len());
    for c in cands {
        if let Ok((next_fen, _stm)) = apply_uci(&fen_key, &c.uci) {
            // push_node equivalent under the lock
            let mut nodes_guard = nodes.lock().await;
            let new_id = nodes_guard.len() as u64;
            let child = RepertoireNode {
                id: new_id,
                parent: Some(node_id),
                fen_key: next_fen,
                last_move_uci: Some(c.uci.clone()),
                ply_depth: ply_depth + 1,
                children: Vec::new(),
                signals: Default::default(),
            };
            nodes_guard.push(child);
            nodes_guard[node_id as usize].children.push(new_id);
            new_child_ids.push(new_id);
        }
    }

    // Enqueue children for further expansion
    for cid in new_child_ids {
        sender.send(cid).await.ok();
    }

    Ok(())
}

// ------------- small helpers (pure) ----------------

fn apply_uci(fen_key: &FenKey, uci: &str) -> Result<(FenKey, Color)> {
    let pos: Chess = fen_key
        .fen_string
        .parse::<shakmaty::fen::Fen>()?
        .into_position(Standard)?;
    let u: Uci = uci.parse()?;
    let m: shakmaty::Move = u.to_move(&pos).map_err(|_| anyhow!("illegal UCI"))?;
    let mut next = pos.clone();
    next.play_unchecked(&m);
    let next_fen = shakmaty::fen::Fen::from_position(next.clone(), Legal).to_string();
    Ok((
        FenKey {
            fen_string: next_fen,
            side_to_move: PieceColor::from_shakmaty(next.turn()),
        },
        next.turn(),
    ))
}
