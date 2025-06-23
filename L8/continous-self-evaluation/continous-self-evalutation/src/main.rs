mod evaluator;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use evaluator::{Evaluator, SystemMetrics};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static EVAL: Lazy<Mutex<Evaluator>> = Lazy::new(|| Mutex::new(Evaluator::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/metrics", post(submit_metrics))
        .route("/evaluate", post(run_evaluation))
        .route("/history", get(get_history));

    println!("Continuous Self-Evaluation running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /metrics
async fn submit_metrics(Json(m): Json<SystemMetrics>) -> impl IntoResponse {
    let mut eval = EVAL.lock().unwrap();
    eval.push_metrics(m);
    Json("metrics recorded")
}

// POST /evaluate
async fn run_evaluation() -> impl IntoResponse {
    let mut eval = EVAL.lock().unwrap();
    let result = eval.evaluate();
    Json(result)
}

// GET /history
async fn get_history() -> impl IntoResponse {
    let eval = EVAL.lock().unwrap();
    Json(eval.history())
}
