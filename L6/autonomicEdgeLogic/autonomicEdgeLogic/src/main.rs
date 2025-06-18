mod edge;

use axum::{
    extract::{Query, Json},
    routing::{get, post},
    Router,
    response::IntoResponse,
};
use edge::{EdgeNode, Decision};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use tracing_subscriber;

static EDGE: Lazy<Mutex<EdgeNode>> = Lazy::new(|| Mutex::new(EdgeNode::new(5)));

#[derive(Debug, Deserialize)]
struct Req {
    ip: String,
    key: String,
}

#[derive(Debug, Serialize)]
struct EdgeResponse {
    ip: String,
    key: String,
    decision: String,
}

// Simulate a request at the edge: local cache, block, or reroute
async fn edge_request(Query(req): Query<Req>) -> impl IntoResponse {
    let mut edge = EDGE.lock().unwrap();
    let decision = edge.handle_request(&req.ip, &req.key);
    let decision_str = match decision {
        Decision::CacheHit => "CacheHit",
        Decision::CacheMissAndFetched => "CacheMissAndFetched",
        Decision::Blocked => "Blocked",
        Decision::Rerouted => "Rerouted",
    }.to_string();

    match decision {
        Decision::Blocked => warn!("Edge blocked IP {} for key {}", req.ip, req.key),
        Decision::Rerouted => warn!("Edge rerouted request from IP {} for key {}", req.ip, req.key),
        Decision::CacheHit => info!("Edge served cache for {} / {}", req.ip, req.key),
        Decision::CacheMissAndFetched => info!("Edge fetched and cached {} / {}", req.ip, req.key),
    }

    Json(EdgeResponse {
        ip: req.ip,
        key: req.key,
        decision: decision_str,
    })
}

// POST /block_ip { "ip": "1.2.3.4" }
#[derive(Debug, Deserialize)]
struct BlockIp { ip: String } // <-- fixed: use comma, not semicolon

async fn block_ip(Json(b): Json<BlockIp>) -> impl IntoResponse { // <-- fixed: mark as async
    let mut edge = EDGE.lock().unwrap();
    edge.block_ip(b.ip.clone());
    Json(serde_json::json!({ "status": "blocked", "ip": b.ip }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/edge", get(edge_request))
        .route("/block_ip", post(block_ip));

    println!("Edge node listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
