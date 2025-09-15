use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{domain::RepertoireNode, search::arena::NodeArenaStore};

/// In-memory arena backed by Arc<Mutex<Vec<RepertoireNode>>>
#[derive(Clone, Default)]
pub struct MemArena {
    inner: Arc<Mutex<Vec<RepertoireNode>>>,
}

impl MemArena {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl NodeArenaStore for MemArena {
    async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }

    async fn get(&self, id: u64) -> Option<RepertoireNode> {
        self.inner.lock().await.get(id as usize).cloned()
    }

    async fn push(&self, node: RepertoireNode) -> u64 {
        let mut g = self.inner.lock().await;
        let id = g.len() as u64;
        let mut n = node;
        n.id = id;
        g.push(n);
        id
    }

    async fn push_child(&self, parent: u64, child_id: u64) {
        let mut g = self.inner.lock().await;
        if let Some(p) = g.get_mut(parent as usize) {
            p.children.push(child_id);
        }
    }
}
