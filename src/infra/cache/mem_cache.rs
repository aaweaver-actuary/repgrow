use async_trait::async_trait;
use moka::future::Cache;
use std::{fmt::Debug, hash::Hash, sync::Arc, time::Duration};

use crate::infra::cache::kv_cache::KvCache;

/// Moka-backed in-memory cache.
#[derive(Debug)]
pub struct MemCache<K, V>
where
    K: Eq + Hash + Send + Sync + Debug + Clone + 'static,
    V: Send + Sync + Debug + 'static,
{
    inner: Cache<K, Arc<V>>,
}

impl<K, V> MemCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
{
    pub fn new(capacity: u64, ttl_secs: u64) -> Self {
        let inner = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();
        Self { inner }
    }
}

#[async_trait]
impl<K, V> KvCache<K, V> for MemCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
{
    async fn get(&self, k: &K) -> Option<Arc<V>> {
        self.inner.get(k).await
    }
    async fn put(&self, k: K, v: Arc<V>) {
        self.inner.insert(k, v).await;
    }
}
