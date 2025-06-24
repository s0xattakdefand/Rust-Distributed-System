//! Horizontal auto-scaler simulator.

use anyhow::Result;
use tokio::time::{interval, Duration};

pub struct Hpa<M, S>
where
    M: Fn() -> f32 + Send + Sync + 'static,
    S: Fn(u32)    + Send + Sync + 'static,
{
    pub min: u32,
    pub max: u32,
    pub target_cpu: f32,
    read_cpu:  M,
    set_scale: S,
}

impl<M, S> Hpa<M, S>
where
    M: Fn() -> f32 + Send + Sync + 'static,
    S: Fn(u32)    + Send + Sync + 'static,
{
    pub async fn run(self, tick: Duration) -> Result<()> {  // ← tick no longer mut
        let mut poll = interval(tick);
        let mut replicas = self.min;

        loop {
            poll.tick().await;
            let cpu = (self.read_cpu)();

            // naive proportional control
            if cpu > self.target_cpu * 1.1 && replicas < self.max {
                replicas += 1;
            } else if cpu < self.target_cpu * 0.7 && replicas > self.min {
                replicas -= 1;
            }
            (self.set_scale)(replicas);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn scales_up_and_down() {
        let reported = Arc::new(Mutex::new(vec![0.9, 0.9, 0.3, 0.3])); // 2 high → 2 low
        let scaled   = Arc::new(Mutex::new(Vec::new()));

        let cpu_r = Arc::clone(&reported);
        let scale = Arc::clone(&scaled);

        let hpa = Hpa {
            min: 1, max: 3, target_cpu: 0.5,
            read_cpu: move || cpu_r.lock().unwrap().remove(0),
            set_scale: move |r| scale.lock().unwrap().push(r),
        };

        tokio::spawn(hpa.run(Duration::from_millis(1)));
        tokio::time::sleep(Duration::from_millis(5)).await;

        let log = scaled.lock().unwrap().clone();
        assert!(log.contains(&2)); // scaled up
        assert!(log.contains(&1)); // scaled back down
    }
}
