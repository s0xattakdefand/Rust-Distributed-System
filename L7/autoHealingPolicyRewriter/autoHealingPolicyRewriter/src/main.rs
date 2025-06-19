mod policy;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rand::Rng;
use policy::Policy;
use tracing_subscriber;

static POLICY: Lazy<Mutex<Policy>> = Lazy::new(|| Mutex::new(Policy::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/call_backend", get(call_backend))
        .route("/policy", get(policy_status))
        .route("/reset", get(reset_policy));

    println!("Auto-Healing Policy Rewriter running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /call_backend
async fn call_backend() -> impl IntoResponse {
    let mut pol = POLICY.lock().unwrap();
    // Simulate a call: 75% chance success, 25% transient failure
    let success = rand::thread_rng().gen_bool(0.75);
    pol.record_result(success);
    pol.auto_heal();
    Json(serde_json::json!({
        "call_result": if success { "success" } else { "transient_failure" },
        "current_policy": &*pol
    }))
}

// GET /policy
async fn policy_status() -> impl IntoResponse {
    let pol = POLICY.lock().unwrap();
    Json(pol.clone())
}

// GET /reset
async fn reset_policy() -> impl IntoResponse {
    let mut pol = POLICY.lock().unwrap();
    pol.reset();
    Json(serde_json::json!({"status":"reset", "policy": &*pol}))
}
