//! Blue/Green deployment helper.
//
// * blue   → currently receiving traffic
// * green  → candidate
//
// In real life you’d PATCH the Kubernetes Service selector / Istio
// VirtualService.  Here we just log & track state.

use anyhow::Result;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color { Blue, Green }

#[derive(Debug)]
pub struct Switcher {
    live: Color,
}

impl Switcher {
    pub fn new(live: Color) -> Self { Self { live } }

    /// Flip live traffic to the other colour, wait `delay`, then verify.
    pub async fn promote(&mut self, delay: Duration) -> Result<()> {
        let target = match self.live { Color::Blue => Color::Green, Color::Green => Color::Blue };
        println!("🔀 shifting traffic to {target:?}");
        // TODO: real Kubernetes patch
        sleep(delay).await;
        // TODO: health-check target colour
        println!("✅ {target:?} looks healthy → commit");
        self.live = target;
        Ok(())
    }

    pub fn live_color(&self) -> Color { self.live }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn flips() {
        let mut s = Switcher::new(Color::Blue);
        s.promote(Duration::from_millis(1)).await.unwrap();
        assert_eq!(s.live_color(), Color::Green);
    }
}
