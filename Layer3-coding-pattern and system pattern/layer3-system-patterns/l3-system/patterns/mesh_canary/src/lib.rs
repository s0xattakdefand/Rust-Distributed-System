//! Progressive rollout simulator (same code style as earlier Layer-2 canary).

use anyhow::{Result, anyhow};
use tokio::time::{sleep, Duration};

pub async fn rollout<F>(windows: &[u8], slo: f32, read_err: F) -> Result<()>
where F: Fn() -> f32 {
    for pct in windows {
        sleep(Duration::from_millis(5)).await;
        let e = read_err();
        if e > slo { return Err(anyhow!("failed at {pct}%")); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn fails() {
        assert!(rollout(&[10,30],0.05,||0.1).await.is_err());
    }
}
