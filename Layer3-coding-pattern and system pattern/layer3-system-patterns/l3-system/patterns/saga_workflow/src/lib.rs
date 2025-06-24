//! Orchestrator service using Layer-2 saga crate.

use layer2_saga::{Step, run_saga};
use anyhow::Result;

pub async fn purchase() -> Result<()> {
    run_saga(vec![
        Step::new(|| reserve_stock(), || unreserve_stock()),
        Step::new(|| capture_payment(),|| refund_payment()),
    ]).await
}

// --- dummy actions ----
fn reserve_stock()   -> Result<()> { println!("stock ok"); Ok(()) }
fn unreserve_stock() -> Result<()> { println!("rollback stock"); Ok(()) }
fn capture_payment() -> Result<()> { Err(anyhow::anyhow!("payment fail")) }
fn refund_payment()  { println!("refund"); }

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn rollback_happens() {
        assert!(purchase().await.is_err());
    }
}
