use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

/// Simulate a flaky operation (e.g. remote API)
async fn try_something() -> Result<String, &'static str> {
    let success = rand::thread_rng().gen_bool(0.3); // 30% chance of success
    if success {
        Ok("🎉 Success!".to_string())
    } else {
        Err("❌ Failed")
    }
}

/// Retry with exponential backoff
async fn retry_with_backoff(max_retries: u32) -> Result<String, &'static str> {
    let mut delay = Duration::from_millis(500); // Start with 500ms

    for attempt in 1..=max_retries {
        println!("🔁 Attempt {}...", attempt);

        match try_something().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                println!("⏳ {} – backing off for {:?}", e, delay);
                sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    }

    Err("🚫 All retries failed")
}

#[tokio::main]
async fn main() {
    match retry_with_backoff(5).await {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}
