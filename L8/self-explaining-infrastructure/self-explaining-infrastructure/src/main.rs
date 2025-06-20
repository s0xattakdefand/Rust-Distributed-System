mod explainer;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use explainer::{SelfExplainingInfra, DeployRequest};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static EXPLAINER: Lazy<Mutex<SelfExplainingInfra>> = Lazy::new(|| Mutex::new(SelfExplainingInfra::new(20)));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/deploy", post(deploy))
        .route("/history", get(history));

    println!("Self-Explaining Infrastructure running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /deploy
async fn deploy(Json(req): Json<DeployRequest>) -> impl IntoResponse {
    let mut infra = EXPLAINER.lock().unwrap();
    let decision = infra.deploy(req);
    Json(decision)
}

// GET /history
async fn history() -> impl IntoResponse {
    let infra = EXPLAINER.lock().unwrap();
    Json(infra.history())
}
