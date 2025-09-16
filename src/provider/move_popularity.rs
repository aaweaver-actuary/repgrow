use async_trait::async_trait;

use crate::{
    domain::{FenKey, PopularityRow},
    provider::PopularityCaps,
};

#[async_trait]
pub trait MovePopularity: Send + Sync {
    async fn sample(&self, fen: &FenKey) -> anyhow::Result<Vec<PopularityRow>>;
    fn caps(&self) -> PopularityCaps;
}
