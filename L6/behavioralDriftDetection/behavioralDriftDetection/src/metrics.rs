use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WINDOW_SIZE: usize = 20;

#[derive(Debug)]
pub struct ResponseTimeWindow {
    times: VecDeque<(Instant, f64)>, // (timestamp, latency_ms)
}

impl ResponseTimeWindow {
    pub fn new() -> Self {
        Self {
            times: VecDeque::with_capacity(WINDOW_SIZE),
        }
    }

    pub fn record(&mut self, latency_ms: f64) {
        let now = Instant::now();
        if self.times.len() == WINDOW_SIZE {
            self.times.pop_front();
        }
        self.times.push_back((now, latency_ms));
    }

    pub fn avg_latency(&self) -> f64 {
        if self.times.is_empty() {
            0.0
        } else {
            self.times.iter().map(|(_, l)| *l).sum::<f64>() / (self.times.len() as f64)
        }
    }
}
