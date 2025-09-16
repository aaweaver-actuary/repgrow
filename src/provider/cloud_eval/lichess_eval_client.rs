//! Lichess Cloud Evaluation provider (engine-quality).
//! Talks to /api/cloud-eval and returns MultiPV lines.

use async_trait::async_trait;

use crate::{
    config::QualityConfig,
    domain::{EvalLine, FenKey},
    provider::{MoveQuality, QualityCaps},
};

pub fn build_lichess_eval_client(
    base_url: &str,
    multi_pv: usize,
    cfg: QualityConfig,
) -> LichessEvalClient {
    LichessEvalClient {
        base_url: base_url.to_string(),
        multi_pv,
        cfg,
    }
}

pub struct LichessEvalClient {
    base_url: String,
    multi_pv: usize,
    cfg: QualityConfig,
}

impl Default for LichessEvalClient {
    fn default() -> Self {
        Self {
            base_url: "https://lichess.org/api/cloud-eval".to_string(),
            multi_pv: 5,
            cfg: QualityConfig::default(),
        }
    }
}

#[async_trait]
impl MoveQuality for LichessEvalClient {
    async fn evaluate(
        &self,
        fen: &FenKey,
        multipv: Option<usize>,
    ) -> anyhow::Result<Vec<EvalLine>> {
        // Uses `multipv` if provided, else defaults to self.multi_pv
        let pv = multipv.unwrap_or(self.multi_pv);

        lichess_eval_api_call(&self.base_url, fen, pv).await
    }

    fn caps(&self) -> QualityCaps {
        QualityCaps {
            max_multipv: self.cfg.multi_pv,
        }
    }
}

async fn lichess_eval_api_call(
    url: &str,
    fen: &FenKey,
    multipv: usize,
) -> anyhow::Result<Vec<EvalLine>> {
    let request_url = build_lichess_eval_url(url, fen, multipv);
    let eval = reqwest::get(&request_url)
        .await?
        .json::<Vec<EvalLine>>()
        .await?;
    Ok(eval)
}

/// Encode fen for URL query param, special-case "startpos" for the starting position.
fn fen_query_param(fen: &FenKey) -> String {
    if fen.fen_string == "startpos" {
        "?fen=startpos".to_string()
    } else {
        format!("?fen={}", urlencoding::encode(&fen.fen_string))
    }
}

/// Encode the multiPv query param.
fn multipv_query_param(multipv: usize) -> String {
    format!("&multiPv={}", multipv)
}

/// Given a FEN string and multiPV, return the full URL for the lichess cloud-eval API call.
fn build_lichess_eval_url(base_url: &str, fen: &FenKey, multipv: usize) -> String {
    let fen_param = fen_query_param(fen);
    let multipv_param = multipv_query_param(multipv);
    format!("{}{}{}", base_url, fen_param, multipv_param)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_lichess_eval_url() {
        let base_url = "https://lichess.org/api/cloud-eval";
        let fen = FenKey {
            fen_string: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            side_to_move: crate::domain::PieceColor::White,
        };
        let multipv = 3;
        let url = build_lichess_eval_url(base_url, &fen, multipv);
        assert_eq!(url, "https://lichess.org/api/cloud-eval?fen=rnbqkbnr%2Fpppppppp%2F8%2F8%2F8%2F8%2FPPPPPPPP%2FRNBQKBNR%20w%20KQkq%20-%200%201&multiPv=3");
    }

    #[tokio::test]
    async fn test_fen_query_param() {
        let fen1 = FenKey {
            fen_string: "startpos".to_string(),
            side_to_move: crate::domain::PieceColor::White,
        };
        assert_eq!(fen_query_param(&fen1), "?fen=startpos");
        let fen2 = FenKey {
            fen_string: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            side_to_move: crate::domain::PieceColor::White,
        };
        assert_eq!(
            fen_query_param(&fen2),
            "?fen=rnbqkbnr%2Fpppppppp%2F8%2F8%2F8%2F8%2FPPPPPPPP%2FRNBQKBNR%20w%20KQkq%20-%200%201"
        );
    }
}
