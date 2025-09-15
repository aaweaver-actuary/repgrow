use repgrow::{provider::{MoveQuality, modname::QualityCaps}, domain::{FenKey}};

struct DummyQ;
#[async_trait::async_trait]
impl MoveQuality for DummyQ {
    async fn evaluate(&self, _fen: &FenKey, _multipv: usize) -> anyhow::Result<Vec<repgrow::domain::EvalLine>> {
        Ok(vec![
            repgrow::domain::EvalLine { uci: "e2e4".into(), eval_cp: 30, depth: 20 },
            repgrow::domain::EvalLine { uci: "d2d4".into(), eval_cp: 10, depth: 20 },
        ])
    }
    fn caps(&self) -> modname::QualityCaps { modname::QualityCaps { max_multipv: 8 } }
}

#[tokio::test]
async fn quality_returns_lines() {
    let q = DummyQ;
    let fen = FenKey { fen: "startpos not real".into(), stm: shakmaty::Color::White };
    let lines = q.evaluate(&fen, 4).await.unwrap();
    assert!(lines.len() >= 1);
}
