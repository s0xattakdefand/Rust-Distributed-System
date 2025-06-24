//! Gateway that fan-outs two backend calls and returns merged JSON.

use actix_web::{get, App, HttpServer, Responder, web::Json};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct Composite { a: String, b: String }

#[get("/aggregate")]
async fn aggregate() -> Result<impl Responder> {
    let a = reqwest::get("https://example.com/a").await?.text().await?;
    let b = reqwest::get("https://example.com/b").await?.text().await?;
    Ok(Json(Composite { a, b }))
}

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(aggregate))
        .bind(("0.0.0.0", 8088))?
        .run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test] async fn comp_serialises() {
        let j = serde_json::to_string(&Composite{a:"x".into(),b:"y".into()}).unwrap();
        assert!(j.contains("\"a\":\"x\""));
    }
}
