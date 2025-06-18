mod scaler;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use scaler::PredictiveScaler;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rand::Rng;
use tracing_subscriber;

static SCALER: Lazy<Mutex<PredictiveScaler>> = Lazy::new(|| Mutex::new(PredictiveScaler::new(10)));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/metrics", get(metrics))
        .route("/simulate", get(simulate_traffic));

    println!("Predictive Auto-Scaler running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /metrics
async fn metrics() -> impl IntoResponse {
    let scaler = SCALER.lock().unwrap();
    Json(scaler.state())
}

// GET /simulate
async fn simulate_traffic() -> impl IntoResponse {
    let mut scaler = SCALER.lock().unwrap();

    // Randomized "traffic" pattern with occasional spike (simulate Black Friday etc)
    let traffic = if rand::thread_rng().gen_bool(0.15) {
        1000 + rand::thread_rng().gen_range(0..800) // spike!
    } else {
        80 + rand::thread_rng().gen_range(0..70)
    };

    scaler.add_traffic(traffic);

    Json(scaler.state())
}
