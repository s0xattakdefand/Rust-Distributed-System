use prometheus::{IntCounter, Encoder, TextEncoder, register_int_counter};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQUESTS: IntCounter = register_int_counter!(
        "app_requests_total", "Total number of requests handled"
    ).unwrap();
    pub static ref REJECTED: IntCounter = register_int_counter!(
        "app_requests_rejected_total", "Total number of requests rejected due to budget"
    ).unwrap();
}

pub fn render_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
