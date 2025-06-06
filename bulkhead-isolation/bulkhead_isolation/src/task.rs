use tokio::time::{sleep, Duration};

pub async fn simulated_task(task_type: &str, id: usize, secs: u64) {
    println!("🚀 Running {} task #{} for {}s", task_type, id, secs);
    sleep(Duration::from_secs(secs)).await;
    println!("✅ Finished {} task #{}", task_type, id);
}
