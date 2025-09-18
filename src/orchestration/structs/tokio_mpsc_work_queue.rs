use tokio::sync::{
    Mutex,
    mpsc::{Receiver, Sender},
};

use crate::orchestration::WorkQueuePort;

pub struct TokioMpscWorkQueue {
    tx: Sender<u64>,
    rx: Mutex<Receiver<u64>>,
}
#[async_trait::async_trait]
impl WorkQueuePort for TokioMpscWorkQueue {
    async fn send(&self, id: u64) -> anyhow::Result<()> {
        self.tx.send(id).await.map_err(Into::into)
    }
    async fn recv(&self) -> Option<u64> {
        self.rx.lock().await.recv().await
    }
    fn close(&self) {
        self.tx.close_channel();
    }
}
