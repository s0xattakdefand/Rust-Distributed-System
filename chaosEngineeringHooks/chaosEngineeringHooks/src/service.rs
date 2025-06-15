use crate::chaos::inject_fault;
use tracing::{info, instrument};
use tokio::time::{sleep, Duration};

#[instrument]
pub async fn resilient_service_operation(request_id: usize) {
    match inject_fault(0.3) {
        Ok(_) => {
            info!("Operation {} succeeded.", request_id);
            sleep(Duration::from_millis(200)).await;
        }
        Err(e) => {
            info!("Operation {} failed due to chaos injection: {}", request_id, e);
        }
    }
}
