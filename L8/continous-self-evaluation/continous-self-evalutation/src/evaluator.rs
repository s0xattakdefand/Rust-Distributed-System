use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub error_rate: f64,    // e.g., 0.01 for 1%
    pub avg_latency_ms: f64,
    pub budget_used: f64,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    pub timestamp: DateTime<Utc>,
    pub score: f64,
    pub status: String, // "healthy", "at risk", "tune_needed"
    pub details: String,
}

pub struct Evaluator {
    pub metrics: Vec<SystemMetrics>,
    pub history: Vec<EvalResult>,
    pub target_error_rate: f64,
    pub target_latency: f64,
    pub target_budget: f64,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            history: Vec::new(),
            target_error_rate: 0.01,
            target_latency: 100.0,
            target_budget: 1000.0,
        }
    }

    pub fn push_metrics(&mut self, m: SystemMetrics) {
        self.metrics.push(m);
    }

    pub fn evaluate(&mut self) -> EvalResult {
        let now = Utc::now();
        let n = self.metrics.len().max(1) as f64;
        let avg_error = self.metrics.iter().map(|m| m.error_rate).sum::<f64>() / n;
        let avg_lat = self.metrics.iter().map(|m| m.avg_latency_ms).sum::<f64>() / n;
        let total_cost = self.metrics.iter().map(|m| m.cost_usd).sum::<f64>();
        let total_budget = self.metrics.iter().map(|m| m.budget_used).sum::<f64>();

        // Simple scoring: 100 - (penalties for exceeding targets)
        let mut score = 100.0;
        let mut details = vec![];
        if avg_error > self.target_error_rate {
            score -= (avg_error - self.target_error_rate) * 1000.0;
            details.push(format!(
                "Error rate {:.2}% over target {:.2}%",
                avg_error * 100.0, self.target_error_rate * 100.0
            ));
        }
        if avg_lat > self.target_latency {
            score -= (avg_lat - self.target_latency) * 0.2;
            details.push(format!(
                "Avg latency {:.1}ms over target {:.1}ms",
                avg_lat, self.target_latency
            ));
        }
        if total_budget > self.target_budget {
            score -= (total_budget - self.target_budget) * 0.1;
            details.push(format!(
                "Budget used ${:.2} over target ${:.2}",
                total_budget, self.target_budget
            ));
        }
        if total_cost > self.target_budget {
            score -= (total_cost - self.target_budget) * 0.1;
            details.push(format!(
                "Total cost ${:.2} over budget target ${:.2}",
                total_cost, self.target_budget
            ));
        }

        let status = if score > 90.0 {
            "healthy"
        } else if score > 70.0 {
            "at risk"
        } else {
            "tune_needed"
        }
        .to_string();

        let details = if details.is_empty() {
            "All targets met".to_string()
        } else {
            details.join("; ")
        };

        let result = EvalResult {
            timestamp: now,
            score,
            status,
            details,
        };
        self.history.push(result.clone());
        result
    }

    pub fn history(&self) -> Vec<EvalResult> {
        self.history.clone()
    }
}
