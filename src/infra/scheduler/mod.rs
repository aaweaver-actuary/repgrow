use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait TaskScheduler: Send + Sync {
    async fn spawn<F>(&self, fut: F) where F: std::future::Future<Output = ()> + Send + 'static;
    fn channel<T: Send + 'static>(&self, cap: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>);
}

#[derive(Clone, Default)]
pub struct TokioScheduler;

#[async_trait]
impl TaskScheduler for TokioScheduler {
    async fn spawn<F>(&self, fut: F) where F: std::future::Future<Output = ()> + Send + 'static {
        tokio::spawn(fut);
    }
    fn channel<T: Send + 'static>(&self, cap: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
        mpsc::channel(cap)
    }
}
