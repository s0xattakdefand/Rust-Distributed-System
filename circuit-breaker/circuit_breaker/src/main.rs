mod breaker;
mod service;

use breaker::CircuitBreaker;
use service::unreliable_service;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut breaker = CircuitBreaker::new(3, 5); // 3 failures, wait 5 seconds

    for attempt in 1..20 {
        println!("🔁 Attempt {}", attempt);

        if breaker.can_call() {
            match unreliable_service().await {
                Ok(success) => {
                    println!("{}", success);
                    breaker.record_success();
                }
                Err(err) => {
                    println!("{}", err);
                    breaker.record_failure();
                }
            }
        } else {
            println!("⛔ Circuit Open - Skipping Call");
        }

        sleep(Duration::from_secs(1)).await;
    }
}
