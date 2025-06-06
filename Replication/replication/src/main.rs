mod replica;
mod types;

use axum::{routing::post, Router, Json};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use types::Data;
use replica::ReplicaSet;

#[tokio::main]
async fn main() {
    let replicas = Arc::new(Mutex::new(ReplicaSet::new(3)));

    let app = Router::new()
        .route("/replicate", post({
            let replicas = replicas.clone();
            move |Json(payload): Json<Data>| {
                let replicas = replicas.clone();
                async move {
                    let mut guard = replicas.lock().unwrap();
                    guard.replicate(payload.clone());
                    Json(format!("Replicated: {:?}", payload))
                }
            }
        }));

    let addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    println!("🚀 Replication server running on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
