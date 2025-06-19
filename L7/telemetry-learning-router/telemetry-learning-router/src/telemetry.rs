use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BackendTelemetry {
    pub backend: String,
    pub count: u32,
    pub successes: u32,
    pub errors: u32,
    pub total_latency_ms: u32,
}

impl BackendTelemetry {
    pub fn new(name: &str) -> Self {
        Self {
            backend: name.to_string(),
            count: 0,
            successes: 0,
            errors: 0,
            total_latency_ms: 0,
        }
    }

    pub fn record(&mut self, latency_ms: u32, success: bool) {
        self.count += 1;
        self.total_latency_ms += latency_ms;
        if success {
            self.successes += 1;
        } else {
            self.errors += 1;
        }
    }

    pub fn avg_latency(&self) -> f32 {
        if self.count > 0 {
            self.total_latency_ms as f32 / self.count as f32
        } else {
            0.0
        }
    }

    pub fn error_rate(&self) -> f32 {
        if self.count > 0 {
            self.errors as f32 / self.count as f32
        } else {
            0.0
        }
    }
}
