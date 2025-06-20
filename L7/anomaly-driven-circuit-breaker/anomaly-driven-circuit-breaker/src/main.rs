mod breaker;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rand::Rng;
use breaker::AnomalyBreaker;
use tracing_subscriber;

static BREAKER: Lazy<Mutex<AnomalyBreaker>> = Lazy::new(|| Mutex::new(AnomalyBreaker::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/call_backend", get(call_backend))
        .route("/breaker", get(breaker_status))
        .route("/reset", get(reset_breaker));

    println!("Anomaly-Driven Circuit Breaker running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /call_backend
async fn call_backend() -> impl IntoResponse {
    let mut breaker = BREAKER.lock().unwrap();

    // Simulate: 80% chance success, 20% failure; anomaly = latency outlier
    let success = rand::thread_rng().gen_bool(0.80);
    let anomaly = if rand::thread_rng().gen_bool(0.12) {
        rand::thread_rng().gen_range(0.85..1.0)
    } else {
        rand::thread_rng().gen_range(0.0..0.6)
    };
    breaker.record(success, anomaly);

    Json(breaker.state())
}

// GET /breaker
async fn breaker_status() -> impl IntoResponse {
    let breaker = BREAKER.lock().unwrap();
    Json(breaker.state())
}

// GET /reset
async fn reset_breaker() -> impl IntoResponse {
    let mut breaker = BREAKER.lock().unwrap();
    breaker.reset();
    Json(breaker.state())
}
