mod orchestrator;

use axum::{
    routing::post,
    Json, Router, response::IntoResponse,
};
use orchestrator::{Region, OrchestrateIntent, pick_region};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

// Example region data: could come from DB or cloud API in real life
static REGIONS: Lazy<Vec<Region>> = Lazy::new(|| vec![
    Region { name: "us-west".into(), cost: 1.10, availability: 0.995, is_eu: false },
    Region { name: "eu-central".into(), cost: 1.25, availability: 0.999, is_eu: true },
    Region { name: "ap-south".into(), cost: 0.95, availability: 0.990, is_eu: false },
    Region { name: "eu-west".into(), cost: 1.22, availability: 0.998, is_eu: true },
]);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/orchestrate", post(orchestrate));
    println!("Intent-Based Orchestrator running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /orchestrate { "goal": "min_cost" }
async fn orchestrate(Json(intent): Json<OrchestrateIntent>) -> impl IntoResponse {
    let decision = pick_region(&REGIONS, &intent);
    Json(decision)
}
