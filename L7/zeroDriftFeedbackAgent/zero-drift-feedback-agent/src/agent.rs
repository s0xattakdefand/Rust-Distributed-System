use rand::Rng;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AgentState {
    pub baseline_fraud_rate: f32,
    pub current_fraud_rate: f32,
    pub retrain_count: u32,
    pub last_action: String,
}

pub struct ZeroDriftAgent {
    pub baseline_fraud_rate: f32,
    pub fraud_history: Vec<f32>,
    pub retrain_count: u32,
    pub last_action: String,
}

impl ZeroDriftAgent {
    pub fn new(baseline: f32) -> Self {
        Self {
            baseline_fraud_rate: baseline,
            fraud_history: Vec::new(),
            retrain_count: 0,
            last_action: "init".to_string(),
        }
    }

    pub fn simulate_tx(&mut self) {
        // Simulate fraud rate drifting up/down randomly
        let drift: f32 = rand::thread_rng().gen_range(-0.02..0.04);
        let last = self.fraud_history.last().cloned().unwrap_or(self.baseline_fraud_rate);
        let mut current = (last + drift).clamp(0.0, 1.0);
        // Add randomness: 90% small drift, 10% big spike
        if rand::thread_rng().gen_bool(0.1) {
            current += rand::thread_rng().gen_range(0.1..0.2);
            current = current.clamp(0.0, 1.0);
        }
        self.fraud_history.push(current);
    }

    pub fn feedback_loop(&mut self) {
        // Compare last 10 entries to baseline
        let n = self.fraud_history.len();
        let recent: Vec<f32> = self.fraud_history.iter().skip(n.saturating_sub(10)).cloned().collect();
        let avg: f32 = if !recent.is_empty() {
            recent.iter().sum::<f32>() / recent.len() as f32
        } else {
            self.baseline_fraud_rate
        };

        // Feedback logic: retrain if avg drifts 2x baseline
        if avg > self.baseline_fraud_rate * 2.0 {
            self.retrain_count += 1;
            self.fraud_history.push(self.baseline_fraud_rate); // "reset" to baseline
            self.last_action = format!("retrain (drift detected, avg {:.3} > baseline {:.3})", avg, self.baseline_fraud_rate);
        } else {
            self.last_action = "ok (no drift)".to_string();
        }
    }

    pub fn state(&self) -> AgentState {
        AgentState {
            baseline_fraud_rate: self.baseline_fraud_rate,
            current_fraud_rate: *self.fraud_history.last().unwrap_or(&self.baseline_fraud_rate),
            retrain_count: self.retrain_count,
            last_action: self.last_action.clone(),
        }
    }
}
