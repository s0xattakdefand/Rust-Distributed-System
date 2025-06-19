mod agent;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use agent::ZeroDriftAgent;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static AGENT: Lazy<Mutex<ZeroDriftAgent>> = Lazy::new(|| Mutex::new(ZeroDriftAgent::new(0.02)));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/simulate", get(simulate))
        .route("/status", get(status));

    println!("Zero-Drift Feedback Agent running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /simulate
async fn simulate() -> impl IntoResponse {
    let mut agent = AGENT.lock().unwrap();
    // Simulate 20 txs, then feedback loop
    for _ in 0..20 {
        agent.simulate_tx();
    }
    agent.feedback_loop();

    Json(agent.state())
}

// GET /status
async fn status() -> impl IntoResponse {
    let agent = AGENT.lock().unwrap();
    Json(agent.state())
}
