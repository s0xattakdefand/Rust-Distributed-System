use reqwest::Client;
use std::error::Error;

pub async fn call_service_b(token: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let res = client
        .get("http://localhost:3000/service-a")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
