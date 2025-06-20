mod config_manager;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use config_manager::SelfTuningConfigManager;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rand::{Rng, distributions::Alphanumeric};
use tracing_subscriber;

static CONFIG: Lazy<Mutex<SelfTuningConfigManager>> = Lazy::new(|| Mutex::new(SelfTuningConfigManager::new(10)));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/status", get(status))
        .route("/simulate", get(simulate));

    println!("Self-Tuning Config Manager running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// GET /status
async fn status() -> impl IntoResponse {
    let cfg = CONFIG.lock().unwrap();
    Json(cfg.get_state())
}

// GET /simulate
async fn simulate() -> impl IntoResponse {
    let mut cfg = CONFIG.lock().unwrap();

    // Simulate 20 cache accesses with random keys (some repeated)
    for _ in 0..20 {
        let key = if rand::thread_rng().gen_bool(0.4) {
            // Repeat a popular key
            "hot".to_string()
        } else {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect::<String>()
        };
        cfg.access(&key);
    }

    // Run feedback loop to tune config based on observed hit/miss rate
    cfg.feedback_tune();

    Json(cfg.get_state())
}
