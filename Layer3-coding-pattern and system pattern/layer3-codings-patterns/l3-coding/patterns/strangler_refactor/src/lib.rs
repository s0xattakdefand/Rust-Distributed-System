//! Strangler-Fig: façade that calls legacy service until Rust slice replaces it.

use anyhow::Result;

/// Facade entry-point.
pub async fn get_user(id: u32) -> Result<String> {
    // New Rust implementation exists for id==42; else call legacy HTTP.
    if id == 42 {
        Ok("local-rust-user".into())
    } else {
        let url = format!("https://legacy.example.com/user/{id}");
        let txt = reqwest::get(url).await?.text().await?;
        Ok(txt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn local_branch() {
        assert_eq!(get_user(42).await.unwrap(), "local-rust-user");
    }
}
