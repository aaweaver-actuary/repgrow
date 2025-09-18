use anyhow::Result;

use crate::orchestration::WorkQueuePort;

pub struct ChildEnqueuer<'a> {
    pub queue: &'a dyn WorkQueuePort,
}

impl<'a> ChildEnqueuer<'a> {
    pub async fn enqueue_all(&self, child_ids: &[u64]) -> Result<()> {
        for &cid in child_ids {
            self.queue.send(cid).await?;
        }
        Ok(())
    }
}

// Unit test: feed [1,2,3]; mock queue recording order; assert FIFO and count.
