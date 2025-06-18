use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Intent {
    pub desired_latency_ms: f64,
}
