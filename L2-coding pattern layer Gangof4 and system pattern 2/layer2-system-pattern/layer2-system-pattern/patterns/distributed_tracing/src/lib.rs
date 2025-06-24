//! OpenTelemetry tracer stub that prints spans.

pub fn init() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .init();
}

#[tracing::instrument]
pub fn expensive() -> i32 { 42 }

#[cfg(test)]
mod tests { #[test] fn span(){super::init(); super::expensive();} }
