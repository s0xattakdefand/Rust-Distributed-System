use service_discovery_pattern::service_discovery::discover::discover_service;
use reqwest::Client;

#[tokio::main]
async fn main() {
    println!("🔍 Order service starting...");

    match discover_service("inventory").await {
        Some(url) => {
            println!("📡 Found inventory at {}", url);
            let client = Client::new();

            match client.get(&url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(body) => println!("✅ Inventory Response: {}", body),
                    Err(e) => eprintln!("❌ Failed to read response body: {:?}", e),
                },
                Err(e) => eprintln!("❌ Request failed: {:?}", e),
            }
        }
        None => {
            println!("❌ Could not discover 'inventory' service.");
        }
    }
}
