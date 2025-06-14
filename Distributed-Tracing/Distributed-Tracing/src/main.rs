use distributed_tracing::{service, tracing::init_tracing};
use tokio::task;

#[tokio::main]
async fn main() {
    init_tracing();

    let mut handles = Vec::new();

    for request_id in 1..=5 {
        handles.push(task::spawn(service::process_request(request_id)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
