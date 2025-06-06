mod bulkhead;
mod task;

use bulkhead::Bulkhead;
use task::simulated_task;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let fast_bulkhead = Arc::new(Bulkhead::new(2)); // For fast tasks
    let slow_bulkhead = Arc::new(Bulkhead::new(1)); // For slow tasks

    let mut handles = vec![];

    // Spawn fast tasks
    for i in 0..5 {
        let bulkhead = fast_bulkhead.clone();
        handles.push(tokio::spawn(async move {
            bulkhead.execute(|| simulated_task("fast", i, 1)).await;
        }));
    }

    // Spawn slow tasks
    for i in 0..3 {
        let bulkhead = slow_bulkhead.clone();
        handles.push(tokio::spawn(async move {
            bulkhead.execute(|| simulated_task("slow", i, 3)).await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("✅ All tasks completed.");
}
