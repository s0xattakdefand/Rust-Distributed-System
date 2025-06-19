use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Policy {
    pub retry_count: u32,
    pub backoff_ms: u32,
    pub circuit_breaker: bool,
    pub fallback_enabled: bool,
    pub failure_window: Vec<bool>, // true = fail, false = success
}

impl Policy {
    pub fn new() -> Self {
        Self {
            retry_count: 3,
            backoff_ms: 50,
            circuit_breaker: false,
            fallback_enabled: false,
            failure_window: vec![],
        }
    }

    pub fn record_result(&mut self, success: bool) {
        self.failure_window.push(!success);
        if self.failure_window.len() > 20 {
            self.failure_window.remove(0);
        }
    }

    pub fn auto_heal(&mut self) {
        let recent_fails = self.failure_window.iter().filter(|x| **x).count();

        // If >40% failures in last 20 calls, enable circuit breaker and increase backoff
        if recent_fails > 8 {
            self.circuit_breaker = true;
            self.backoff_ms = 200;
            self.retry_count = 1;
            self.fallback_enabled = true;
        } else if recent_fails > 4 {
            // More moderate: increase backoff, fewer retries
            self.backoff_ms = 100;
            self.retry_count = 2;
            self.circuit_breaker = false;
            self.fallback_enabled = false;
        } else {
            // Healthy: keep default
            self.retry_count = 3;
            self.backoff_ms = 50;
            self.circuit_breaker = false;
            self.fallback_enabled = false;
        }
    }

    pub fn reset(&mut self) {
        self.failure_window.clear();
        self.retry_count = 3;
        self.backoff_ms = 50;
        self.circuit_breaker = false;
        self.fallback_enabled = false;
    }
}
