mod telemetry;
mod router;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use once_cell::sync::Lazy;
use telemetry::BackendTelemetry;
use std::sync::Mutex;
use rand::Rng;
use tracing_subscriber;

static BACKENDS: &[&str] = &["aws", "gcp", "azure", "onprem"];

static TELEMETRY: Lazy<Mutex<Vec<BackendTelemetry>>> = Lazy::new(|| {
    Mutex::new(BACKENDS.iter().map(|&name| BackendTelemetry::new(name)).collect())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/route", get(route))
        .route("/telemetry", get(telemetry))
        .route("/simulate", get(simulate_traffic));

    println!("Telemetry-Learning Router running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /route
async fn route() -> impl IntoResponse {
    let telemetry = TELEMETRY.lock().unwrap();
    let best = router::pick_best_backend(&telemetry);
    Json(serde_json::json!({ "chosen_backend": best }))
}

// GET /telemetry
async fn telemetry() -> impl IntoResponse {
    let telemetry = TELEMETRY.lock().unwrap();
    Json(telemetry.clone())
}

// GET /simulate
async fn simulate_traffic() -> impl IntoResponse {
    let mut telemetry = TELEMETRY.lock().unwrap();
    // Randomly simulate traffic and update telemetry for each backend
    for backend in telemetry.iter_mut() {
        let latency = rand::thread_rng().gen_range(20..180);
        let success = rand::thread_rng().gen_bool(0.92 - 0.1 * (BACKENDS.iter().position(|&b| b == backend.backend).unwrap() as f64));
        backend.record(latency, success);
    }
    Json(telemetry.clone())
}
