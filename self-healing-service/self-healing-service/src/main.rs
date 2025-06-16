use rand::Rng;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::{info, error};
use tracing_subscriber;

async fn flakey_service_task(id: usize) {
    loop {
        info!("Service task {} is working...", id);
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Call thread_rng() here, inside the loop, so it does not cross .await
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) {
            error!("Service task {} FAILED! (simulated)", id);
            // Exit (simulate crash/panic)
            return;
        }
    }
}

// Watchdog monitors the child task and restarts it if needed
async fn watchdog(id: usize) {
    let mut attempt = 0;
    loop {
        attempt += 1;
        info!("Starting service task {} (attempt #{})", id, attempt);
        let handle: JoinHandle<()> = tokio::spawn(flakey_service_task(id));
        // Wait for the task to finish (fail or exit)
        let _ = handle.await;
        error!("Watchdog: Detected failure in service task {}. Restarting...", id);
        // Loop will restart the service
    }
}

#[tokio::main]
async fn main() {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Launch multiple self-healing service tasks (each with their own watchdog)
    for id in 1..=2 {
        tokio::spawn(watchdog(id));
    }

    // Run forever
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
