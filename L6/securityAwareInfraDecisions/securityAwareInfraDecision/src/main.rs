mod firewall;

use axum::{
    extract::{ConnectInfo, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use firewall::Firewall;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Mutex;
use tracing::{info, warn};
use tracing_subscriber;

static FIREWALL: Lazy<Mutex<Firewall>> = Lazy::new(|| Mutex::new(Firewall::new()));

#[derive(Debug, Deserialize)]
struct BlockRequest {
    ip: String,
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    allowed: bool,
    ip: String,
    message: String,
}

// GET: /protected
async fn protected_endpoint(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    let ip = addr.ip().to_string();
    let fw = FIREWALL.lock().unwrap();
    if fw.is_blocked(&ip) {
        warn!("Blocked suspicious IP: {}", ip);
        (
            axum::http::StatusCode::FORBIDDEN,
            Json(StatusResponse {
                allowed: false,
                ip,
                message: "Access denied: Your IP has been blocked due to suspicious activity".into(),
            }),
        )
    } else {
        info!("Allowed IP: {}", ip);
        (
            axum::http::StatusCode::OK,
            Json(StatusResponse {
                allowed: true,
                ip,
                message: "Access granted: Welcome to the secure service".into(),
            }),
        )
    }
}

// POST: /block_ip { "ip": "127.0.0.1" }
async fn block_ip(Json(req): Json<BlockRequest>) -> impl IntoResponse {
    let mut fw = FIREWALL.lock().unwrap();
    fw.block_ip(req.ip.clone());
    info!("Blocked IP: {}", req.ip);
    Json(serde_json::json!({"status": "blocked", "ip": req.ip}))
}

// POST: /unblock_ip { "ip": "127.0.0.1" }
async fn unblock_ip(Json(req): Json<BlockRequest>) -> impl IntoResponse {
    let mut fw = FIREWALL.lock().unwrap();
    fw.unblock_ip(&req.ip);
    info!("Unblocked IP: {}", req.ip);
    Json(serde_json::json!({"status": "unblocked", "ip": req.ip}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/protected", get(protected_endpoint))
        .route("/block_ip", post(block_ip))
        .route("/unblock_ip", post(unblock_ip))
        .into_make_service_with_connect_info::<SocketAddr>();

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
