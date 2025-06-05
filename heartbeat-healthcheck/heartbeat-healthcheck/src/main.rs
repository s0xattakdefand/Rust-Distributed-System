mod heartbeat;
mod utils;

use axum::{Json, Router, routing::{get, post}};
use heartbeat::HeartbeatManager;
use serde::Deserialize;
use std::sync::Arc;
use tokio::{net::TcpListener, time::{self, Duration}};
use axum::serve;

#[derive(Deserialize)]
struct HeartbeatRequest {
    node_id: String,
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(HeartbeatManager::new(10)); // 10s timeout

    let manager_clone = manager.clone();
    tokio::spawn(async move {
        loop {
            let active = manager_clone.get_active_nodes().await;
            println!("✅ Active Nodes: {:?}", active);
            time::sleep(Duration::from_secs(5)).await;
        }
    });

    let app = Router::new()
        .route("/heartbeat", post({
            let manager = manager.clone();
            move |Json(req): Json<HeartbeatRequest>| {
                let manager = manager.clone();
                async move {
                    manager.receive_heartbeat(req.node_id).await;
                    Json("✅ Heartbeat received.")
                }
            }
        }))
        .route("/active", get({
            let manager = manager.clone();
            move || {
                let manager = manager.clone();
                async move {
                    let list = manager.get_active_nodes().await;
                    Json(list)
                }
            }
        }));

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("🌐 Running on http://127.0.0.1:3001");

    serve(listener, app).await.unwrap(); // ✅ Start the Axum server here
}
