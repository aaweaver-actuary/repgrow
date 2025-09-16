use repgrow::{
    domain::FenKey,
    provider::{MovePopularity, PopularityCaps},
};

struct DummyP;
#[async_trait::async_trait]
impl MovePopularity for DummyP {
    async fn sample(&self, _fen: &FenKey) -> anyhow::Result<Vec<repgrow::domain::PopularityRow>> {
        Ok(vec![
            repgrow::domain::PopularityRow {
                uci: "e2e4".into(),
                play_rate: 0.55,
                games: 1000,
            },
            repgrow::domain::PopularityRow {
                uci: "d2d4".into(),
                play_rate: 0.25,
                games: 600,
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
async fn popularity_returns_rows() {
    let p = DummyP;
    let fen = FenKey {
        fen_string: "startpos not real".into(),
        side_to_move: shakmaty::Color::White,
    };
    let rows = p.sample(&fen).await.unwrap();
    assert!(rows.iter().any(|r| r.play_rate > 0.0));
}
