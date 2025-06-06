mod publisher;
mod subscriber;
mod message;

use axum::{routing::post, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use publisher::publish;
use subscriber::subscribe;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);
    let shared_tx = Arc::new(Mutex::new(tx));

    let app = Router::new()
        .route("/publish", post(publish))
        .route("/subscribe", post(subscribe))
        .with_state(shared_tx);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🚀 Pub/Sub server running on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}