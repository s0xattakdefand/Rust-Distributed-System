//! Simple semaphore bulkhead (permits = N concurrent).

use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Clone)]
pub struct Bulkhead { sem: Arc<Semaphore> }

impl Bulkhead {
    pub fn new(max: usize) -> Self { Self { sem: Arc::new(Semaphore::new(max)) } }
    pub async fn run<F, T>(&self, fut: F) -> T
    where F: std::future::Future<Output = T> {
        let _permit = self.sem.acquire().await.unwrap();
        fut.await
    }
}

#[cfg(test)]
mod tests {
    use super::*; use tokio::time::*;
    #[tokio::test] async fn limits_parallel() {
        let b = Bulkhead::new(1);
        let t1 = b.run(async { sleep(Duration::from_millis(50)).await; 1 });
        let t2 = b.run(async { 2 });
        let (a,b) = tokio::join!(t1,t2);
        assert_eq!(a+b,3);
    }
}
