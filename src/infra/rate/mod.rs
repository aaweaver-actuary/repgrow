use governor::{
    clock::DefaultClock, state::InMemoryState, state::NotKeyed, Quota, RateLimiter as GovLimiter,
};
use std::{num::NonZeroU32, sync::Arc};

/// Minimal rate-limiter interface.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<GovLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimiter {
    pub fn per_sec(n: u32) -> Self {
        let q = Quota::per_second(NonZeroU32::new(n.max(1)).unwrap());
        Self {
            inner: Arc::new(GovLimiter::<NotKeyed, InMemoryState, DefaultClock>::direct(
                q,
            )),
        }
    }
    pub async fn acquire(&self) {
        use governor::Jitter;
        let _ = self
            .inner
            .until_ready_with_jitter(Jitter::up_to(std::time::Duration::from_millis(30)))
            .await;
    }
}
