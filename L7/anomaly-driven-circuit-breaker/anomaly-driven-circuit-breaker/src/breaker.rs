use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BreakerState {
    pub recent_failures: usize,
    pub recent_anomalies: usize,
    pub breaker_open: bool,
    pub anomaly_score: f32,
    pub reason: String,
}

pub struct AnomalyBreaker {
    failures: Vec<bool>,     // true = failure, false = success
    anomalies: Vec<f32>,     // anomaly score for each call
    pub breaker_open: bool,
    pub reason: String,
}

impl AnomalyBreaker {
    pub fn new() -> Self {
        Self {
            failures: Vec::with_capacity(20),
            anomalies: Vec::with_capacity(20),
            breaker_open: false,
            reason: "healthy".to_string(),
        }
    }

    pub fn record(&mut self, success: bool, anomaly: f32) {
        self.failures.push(!success);
        if self.failures.len() > 20 {
            self.failures.remove(0);
        }
        self.anomalies.push(anomaly);
        if self.anomalies.len() > 20 {
            self.anomalies.remove(0);
        }
        self.evaluate();
    }

    fn evaluate(&mut self) {
        let recent_failures = self.failures.iter().filter(|x| **x).count();
        let recent_anomalies = self.anomalies.iter().filter(|x| **x > 0.9).count();
        let anomaly_score: f32 = if self.anomalies.len() > 0 {
            self.anomalies.iter().sum::<f32>() / self.anomalies.len() as f32
        } else {
            0.0
        };

        if recent_failures > 6 {
            self.breaker_open = true;
            self.reason = "Too many failures".to_string();
        } else if recent_anomalies > 3 || anomaly_score > 0.7 {
            self.breaker_open = true;
            self.reason = format!(
                "Anomalies detected (score {:.2})",
                anomaly_score
            );
        } else {
            self.breaker_open = false;
            self.reason = "healthy".to_string();
        }
    }

    pub fn reset(&mut self) {
        self.failures.clear();
        self.anomalies.clear();
        self.breaker_open = false;
        self.reason = "healthy".to_string();
    }

    pub fn state(&self) -> BreakerState {
        BreakerState {
            recent_failures: self.failures.iter().filter(|x| **x).count(),
            recent_anomalies: self.anomalies.iter().filter(|x| **x > 0.9).count(),
            breaker_open: self.breaker_open,
            anomaly_score: if self.anomalies.len() > 0 {
                self.anomalies.iter().sum::<f32>() / self.anomalies.len() as f32
            } else {
                0.0
            },
            reason: self.reason.clone(),
        }
    }
}
