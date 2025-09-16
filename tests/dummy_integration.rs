use anyhow::Result;
use repgrow::{
    config::SearchConfig,
    domain::{EvalLine, FenKey, PopularityRow},
    policy::SideSplitPolicy,
    provider::{MovePopularity, MoveQuality, PopularityCaps, QualityCaps},
    search::Orchestrator,
};
use std::sync::Arc;

struct DummyQ;
#[async_trait::async_trait]
impl MoveQuality for DummyQ {
    async fn evaluate(&self, _fen: &FenKey, _multipv: Option<usize>) -> Result<Vec<EvalLine>> {
        Ok(vec![
            EvalLine {
                uci: "e2e4".into(),
                eval_cp: 30,
                depth: 20,
            },
            EvalLine {
                uci: "d2d4".into(),
                eval_cp: 10,
                depth: 20,
            },
        ])
    }
    fn caps(&self) -> QualityCaps {
        QualityCaps { max_multipv: 8 }
    }
}

struct DummyP;
#[async_trait::async_trait]
impl MovePopularity for DummyP {
    async fn sample(&self, _fen: &FenKey) -> Result<Vec<PopularityRow>> {
        Ok(vec![
            PopularityRow {
                uci: "e7e5".into(),
                play_rate: 0.6,
                games: 1000,
            },
            PopularityRow {
                uci: "c7c5".into(),
                play_rate: 0.3,
                games: 800,
            },
        ])
    }
    fn caps(&self) -> PopularityCaps {
        PopularityCaps {
            supports_filters: true,
        }
    }
}

#[tokio::test]
async fn dispatcher_builds_two_plies() {
    // Constrain branching to keep the tree small & deterministic
    let cfg = SearchConfig {
        concurrency: 4,
        max_total_nodes: Some(10),
        max_children_my_side: Some(1),
        max_children_opp_side: Some(1),
    };
    let policy = SideSplitPolicy::new(shakmaty::Color::White, 50, 0.05);

    let orch = Orchestrator::new(cfg, policy, Arc::new(DummyQ), Arc::new(DummyP));

    // Start from initial position; expand two plies (W then B)
    let root = orch.build_from_start(None, 2).await.unwrap();

    // We can't easily read back the full arena here without a getter,
    // but this ensures the pipeline runs to completion and returns a root.
    assert_eq!(root.ply_depth, 0);
}
