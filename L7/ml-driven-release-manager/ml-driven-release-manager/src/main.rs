mod manager;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use manager::{ReleaseManager, ReleaseInput};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static MANAGER: Lazy<Mutex<ReleaseManager>> = Lazy::new(|| Mutex::new(ReleaseManager::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/propose_release", post(propose_release))
        .route("/releases", get(releases));

    println!("ML-Driven Release Manager running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /propose_release { "version": "1.2.3", "features": ["cache", "async"], "metric_score": 0.8 }
async fn propose_release(Json(input): Json<ReleaseInput>) -> impl IntoResponse {
    let mut mgr = MANAGER.lock().unwrap();
    let record = mgr.propose_release(input);
    Json(record)
}

// GET /releases
async fn releases() -> impl IntoResponse {
    let mgr = MANAGER.lock().unwrap();
    Json(mgr.releases.clone())
}
