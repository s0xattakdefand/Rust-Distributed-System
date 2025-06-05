use axum::{routing::get, Json, Router};
use rand::Rng;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::election::LeaderElection;

pub fn generate_node_id() -> u32 {
    rand::thread_rng().gen_range(100..200)
}

#[derive(Serialize)]
struct LeaderInfo {
    leader_id: Option<u32>,
}

pub async fn launch_web_server(state: Arc<Mutex<LeaderElection>>) {
    let app = Router::new().route(
        "/leader",
        get({
            let state = state.clone();
            move || async move {
                let guard = state.lock().await;
                Json(LeaderInfo {
                    leader_id: guard.get_leader(),
                })
            }
        }),
    );

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("🌐 Web interface running on http://{}", addr);

    // ✅ Correct launch for axum 0.7 with hyper 1.0
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
