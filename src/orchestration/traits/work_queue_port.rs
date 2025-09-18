use async_trait::async_trait;

#[async_trait]
pub trait WorkQueuePort {
    async fn send(&self, node_id: u64) -> anyhow::Result<()>;
    async fn recv(&self) -> Option<u64>;
    fn close(&self);
}
