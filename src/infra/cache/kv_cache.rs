use async_trait::async_trait;
use std::{hash::Hash, sync::Arc};

/// Minimal cache trait to decouple from moka.
#[async_trait]
pub trait KvCache<K, V>: Send + Sync
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    async fn get(&self, k: &K) -> Option<Arc<V>>;
    async fn put(&self, k: K, v: Arc<V>);
}
