use tracing::{info, instrument};
use tokio::time::{sleep, Duration};

#[instrument]
pub async fn process_request(request_id: usize) {
    info!("Started processing request {}", request_id);
    sleep(Duration::from_millis(100)).await;
    info!("Finished processing request {}", request_id);
}
