use crate::domain::RepertoireNode;

#[async_trait::async_trait]
pub trait NodeArenaStore: Send + Sync {
    async fn len(&self) -> usize;
    async fn get(&self, id: u64) -> Option<RepertoireNode>;
    async fn push(&self, node: RepertoireNode) -> u64; // returns id
    async fn push_child(&self, parent: u64, child_id: u64);
}
