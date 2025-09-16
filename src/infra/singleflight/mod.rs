use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

/// Simple single-flight coalescer to dedupe inflight requests per key.
#[derive(Debug)]
pub struct SingleFlight<K, V> {
    inflight: Mutex<HashMap<K, Vec<oneshot::Sender<Arc<V>>>>>,
}

impl<K, V> SingleFlight<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            inflight: Mutex::new(HashMap::new()),
        }
    }

    /// Run f once for key k; concurrent callers await same result.
    pub async fn run<F, Fut>(&self, k: K, f: F) -> anyhow::Result<Arc<V>>
    where
        F: FnOnce(K) -> Fut + Send + 'static,
        Fut: Future<Output = anyhow::Result<Arc<V>>> + Send + 'static,
        V: Send + Sync + 'static,
    {
        let (rx_opt, do_fetch) = {
            let mut g = self.inflight.lock().await;
            if let Some(waiters) = g.get_mut(&k) {
                let (tx, rx) = oneshot::channel();
                waiters.push(tx);
                (Some(rx), false)
            } else {
                g.insert(k.clone(), Vec::new());
                (None, true)
            }
        };

        if !do_fetch {
            return Ok(rx_opt.unwrap().await.expect("inflight canceled"));
        }

        let res = f(k.clone()).await;
        let waiters = {
            let mut g = self.inflight.lock().await;
            g.remove(&k).unwrap_or_default()
        };
        match &res {
            Ok(v) => {
                for w in waiters {
                    let _ = w.send(v.clone());
                }
            }
            Err(_) => {
                // Do not send anything to waiters on error, or optionally send a default value if appropriate.
                // for w in waiters { let _ = w.send(Arc::new(panic!("no value"))); }
            }
        }
        res
    }
}
