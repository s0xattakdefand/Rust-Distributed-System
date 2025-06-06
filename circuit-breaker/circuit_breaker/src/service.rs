use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{sleep, Duration};

static CALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn unreliable_service() -> Result<String, &'static str> {
    let count = CALL_COUNTER.fetch_add(1, Ordering::Relaxed);

    sleep(Duration::from_millis(200)).await;

    if count % 5 == 0 {
        Ok("🎉 Service OK".to_string())
    } else {
        Err("💥 Service Failed")
    }
}
