use async_trait::async_trait;

use crate::{
    domain::{EvalLine, FenKey},
    provider::QualityCaps,
};

#[async_trait]
pub trait MoveQuality: Send + Sync {
    async fn evaluate(&self, fen: &FenKey, multipv: Option<usize>)
        -> anyhow::Result<Vec<EvalLine>>;
    fn caps(&self) -> QualityCaps;
}
