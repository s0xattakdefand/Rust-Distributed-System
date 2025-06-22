mod policy;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use policy::{PolicyState, RequestLog};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;
use chrono::Utc;

static POLICY: Lazy<Mutex<PolicyState>> = Lazy::new(|| Mutex::new(PolicyState::new()));
static REQUESTS: Lazy<Mutex<Vec<RequestLog>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/simulate_request", post(simulate_request))
        .route("/policy", get(get_policy))
        .route("/audit", get(get_audit));

    println!("AI-Augmented Policy Engine running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /simulate_request { "ip":"1.2.3.4", "endpoint":"/login", "risk_score": 0.85 }
async fn simulate_request(Json(mut log): Json<RequestLog>) -> impl IntoResponse {
    log.timestamp = Utc::now();
    {
        let mut reqs = REQUESTS.lock().unwrap();
        reqs.push(log.clone());
        if reqs.len() > 200 { reqs.drain(0..100); } // Keep buffer short
    }
    let mut policy = POLICY.lock().unwrap();
    let reqs = REQUESTS.lock().unwrap();
    policy.evolve(&reqs);
    Json(policy.clone())
}

// GET /policy
async fn get_policy() -> impl IntoResponse {
    let policy = POLICY.lock().unwrap();
    Json(policy.clone())
}

// GET /audit
async fn get_audit() -> impl IntoResponse {
    let policy = POLICY.lock().unwrap();
    Json(policy.audit_trail.clone())
}
