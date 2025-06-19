use crate::telemetry::BackendTelemetry;

pub fn pick_best_backend(backends: &[BackendTelemetry]) -> Option<String> {
    // Lower error rate wins, then lower latency
    backends
        .iter()
        .min_by(|a, b| {
            let error_ord = a.error_rate().partial_cmp(&b.error_rate()).unwrap();
            if error_ord != std::cmp::Ordering::Equal {
                return error_ord;
            }
            a.avg_latency()
                .partial_cmp(&b.avg_latency())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|b| b.backend.clone())
}
