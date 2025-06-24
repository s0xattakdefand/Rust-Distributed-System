//! Simple exponential-backoff HTTP GET.

use anyhow::Result;
use backoff::{backoff::Backoff, ExponentialBackoff};
use reqwest::Client;

pub async fn get_with_backoff(url: &str) -> Result<String> {
    let client = Client::new();
    let mut policy = ExponentialBackoff {
        max_elapsed_time: Some(std::time::Duration::from_secs(15)),
        ..Default::default()
    };

    loop {
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => return Ok(resp.text().await?),
            _ => {
                if let Some(delay) = policy.next_backoff() {
                    tokio::time::sleep(delay).await;
                } else {
                    return Err(anyhow::anyhow!("retry budget exhausted"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fails_fast() {
        // port 1 unlikely to be open → immediate error
        assert!(get_with_backoff("http://localhost:1").await.is_err());
    }
}
