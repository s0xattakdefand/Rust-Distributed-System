use chaos_engineering_hooks::service::resilient_service_operation;
use tracing_subscriber::EnvFilter;
use tokio::task;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut handles = Vec::new();

    for request_id in 1..=10 {
        handles.push(task::spawn(resilient_service_operation(request_id)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
