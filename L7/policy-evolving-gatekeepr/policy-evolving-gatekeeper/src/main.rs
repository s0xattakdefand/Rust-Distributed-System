mod gatekeeper;

use axum::{
    extract::{Query},
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use gatekeeper::Gatekeeper;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rand::Rng;
use std::collections::HashMap;
use tracing_subscriber;

static GATEKEEPER: Lazy<Mutex<Gatekeeper>> = Lazy::new(|| Mutex::new(Gatekeeper::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/call_api", get(call_api))
        .route("/policy", get(get_policy))
        .route("/reset", get(reset_gatekeeper));

    println!("Policy-Evolving Gatekeeper running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /call_api?user_id=abc123
async fn call_api(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let user_id = params.get("user_id").cloned().unwrap_or_else(|| "anonymous".to_string());
    let mut gk = GATEKEEPER.lock().unwrap();

    // Simulate 80% chance success, 20% error for this user
    let success = rand::thread_rng().gen_bool(0.8);
    let policy = gk.call_api(&user_id, success);

    Json(serde_json::json!({
        "user_id": user_id,
        "success": success,
        "policy": policy
    }))
}

// GET /policy?user_id=abc123
async fn get_policy(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let user_id = params.get("user_id").cloned().unwrap_or_else(|| "anonymous".to_string());
    let gk = GATEKEEPER.lock().unwrap();
    let policy = gk.get_policy(&user_id);
    Json(policy)
}

// GET /reset
async fn reset_gatekeeper() -> impl IntoResponse {
    let mut gk = GATEKEEPER.lock().unwrap();
    gk.reset();
    Json(serde_json::json!({"status": "reset"}))
}
