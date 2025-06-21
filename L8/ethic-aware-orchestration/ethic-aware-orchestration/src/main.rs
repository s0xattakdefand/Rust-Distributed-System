mod ethics;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use ethics::{EthicsEngine, DeployIntent};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static ENGINE: Lazy<Mutex<EthicsEngine>> = Lazy::new(|| Mutex::new(EthicsEngine::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/orchestrate", post(orchestrate))
        .route("/constraints", get(constraints));

    println!("Ethics-Aware Orchestrator running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /orchestrate
async fn orchestrate(Json(intent): Json<DeployIntent>) -> impl IntoResponse {
    let engine = ENGINE.lock().unwrap();
    let decision = engine.check_intent(&intent);
    Json(decision)
}

// GET /constraints
async fn constraints() -> impl IntoResponse {
    let engine = ENGINE.lock().unwrap();
    Json(engine.constraints.clone())
}
