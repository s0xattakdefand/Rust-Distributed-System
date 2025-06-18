use std::collections::VecDeque;

const WINDOW: usize = 20;

pub struct LatencyWindow {
    pub latencies: VecDeque<f64>,
}

impl LatencyWindow {
    pub fn new() -> Self {
        Self { latencies: VecDeque::with_capacity(WINDOW) }
    }

    pub fn record(&mut self, value: f64) {
        if self.latencies.len() == WINDOW {
            self.latencies.pop_front();
        }
        self.latencies.push_back(value);
    }

    pub fn avg(&self) -> f64 {
        if self.latencies.is_empty() {
            0.0
        } else {
            self.latencies.iter().sum::<f64>() / self.latencies.len() as f64
        }
    }
}
