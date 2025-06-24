//! Simple canary rollout controller.
//
// Split traffic in progressive percentages and watch an SLO metric
// (here simulated by a callback that returns the current error-rate).

use anyhow::{anyhow, Result};
use tokio::time::{sleep, Duration};

pub struct Canary<R>
where
    R: Fn() -> f32 + Send + Sync + 'static,
{
    pub steps: Vec<u8>, // e.g. [5, 25, 50, 100]
    pub slo:   f32,     // max error-rate allowed
    pub read_error_rate: R,
}

impl<R> Canary<R>
where
    R: Fn() -> f32 + Send + Sync + 'static,
{
    pub async fn execute(&self) -> Result<()> {
        for pct in &self.steps {
            println!("🔄 shifting {pct}% traffic to v2");
            sleep(Duration::from_millis(10)).await; // pretend to wait metrics window
            let err = (self.read_error_rate)();
            println!("ℹ️  observed error-rate = {err:.2}");
            if err > self.slo {
                return Err(anyhow!("❌ error-rate {err:.2} exceeds SLO {s}", s = self.slo));
            }
        }
        println!("✅ canary successful – v2 is now live");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn aborts_on_bad() {
        let c = Canary {
            steps: vec![10, 50],
            slo:   0.05,
            read_error_rate: || 0.08,        // always bad
        };
        assert!(c.execute().await.is_err());
    }
}
