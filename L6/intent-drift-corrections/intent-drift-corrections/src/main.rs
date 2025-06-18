mod intent;
mod monitor;
mod orchestrator;

use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
    response::IntoResponse,
};
use intent::Intent;
use monitor::LatencyWindow;
use orchestrator::{detect_and_correct, Correction};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::{info, warn};
use tracing_subscriber;

static INTENT: Lazy<Mutex<Intent>> = Lazy::new(|| Mutex::new(Intent { desired_latency_ms: 100.0 }));
static WINDOW: Lazy<Mutex<LatencyWindow>> = Lazy::new(|| Mutex::new(LatencyWindow::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/set_intent", post(set_intent))
        .route("/simulate", get(simulate_work))
        .route("/intent_status", get(intent_status));

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// User can set the intent (desired latency)
async fn set_intent(Json(new_intent): Json<Intent>) -> impl IntoResponse {
    let mut i = INTENT.lock().unwrap();
    *i = new_intent;
    Json(serde_json::json!({ "status": "Intent updated", "intent": &*i }))
}

// Simulate a request and record latency
async fn simulate_work() -> impl IntoResponse {
    let latency = if rand::random::<f64>() < 0.7 {
        80.0 + rand::random::<f64>() * 30.0 // usually 80-110ms
    } else {
        200.0 + rand::random::<f64>() * 100.0 // occasionally 200-300ms
    };
    tokio::time::sleep(std::time::Duration::from_millis(latency as u64)).await;
    WINDOW.lock().unwrap().record(latency);
    format!("Simulated request latency: {:.1}ms", latency)
}

// Show drift, intent, and correction if needed
async fn intent_status() -> impl IntoResponse {
    let i = INTENT.lock().unwrap().clone();
    let avg = WINDOW.lock().unwrap().avg();
    let (drifted, correction) = detect_and_correct(i.desired_latency_ms, avg);
    let action = match correction {
        Correction::None => "None",
        Correction::ScaleUp => "ScaleUp (adding capacity)",
        Correction::Reroute => "Reroute (sending traffic elsewhere)",
        Correction::Restart => "Restart (restarting service)",
    };
    if drifted {
        warn!(
            "Intent drift! Desired latency: {:.1}ms, actual avg: {:.1}ms. Correction: {}",
            i.desired_latency_ms, avg, action
        );
    } else {
        info!(
            "Intent met. Desired latency: {:.1}ms, actual avg: {:.1}ms.",
            i.desired_latency_ms, avg
        );
    }
    Json(serde_json::json!({
        "intent": i,
        "avg_latency": avg,
        "drifted": drifted,
        "correction": action,
    }))
}
