mod market;

use axum::{
    routing::{post, get},
    Json, Router, response::IntoResponse,
};
use market::{Market, WorkloadBid};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;
use uuid::Uuid;

static MARKET: Lazy<Mutex<Market>> = Lazy::new(|| Mutex::new(Market::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/submit_workload", post(submit_workload))
        .route("/providers", get(list_providers))
        .route("/marketplace", get(list_marketplace));

    println!("Autonomous Negotiator/Market running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /submit_workload { "requester":"svcA", "cpu_needed":4, "max_price":8.0, "preferred_location":"us-west" }
async fn submit_workload(Json(mut bid): Json<WorkloadBid>) -> impl IntoResponse {
    bid.id = Uuid::new_v4().to_string();
    let mut market = MARKET.lock().unwrap();
    let assigned = market.submit_bid(bid);
    Json(assigned)
}

// GET /providers
async fn list_providers() -> impl IntoResponse {
    let market = MARKET.lock().unwrap();
    Json(market.get_providers())
}

// GET /marketplace
async fn list_marketplace() -> impl IntoResponse {
    let market = MARKET.lock().unwrap();
    Json(market.get_bids())
}
