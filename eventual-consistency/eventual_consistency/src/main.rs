mod api;
mod model;
mod store;
mod background;

use axum::{Router, routing::post};
use std::net::SocketAddr;
use store::SharedStore;
use api::create_user;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(store::InMemoryStore::new()));
    background::spawn_sync_task(store.clone());

    let app = Router::new()
        .route("/create_user", post(create_user))
        .with_state(store);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🌀 Eventual Consistency API at {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
