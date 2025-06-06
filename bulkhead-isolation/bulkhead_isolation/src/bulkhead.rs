use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct Bulkhead {
    semaphore: Arc<Semaphore>,
}

impl Bulkhead {
    pub fn new(limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    pub async fn execute<F, Fut>(&self, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let permit = self.semaphore.acquire().await.unwrap();
        f().await;
        drop(permit); // Release after use
    }
}
