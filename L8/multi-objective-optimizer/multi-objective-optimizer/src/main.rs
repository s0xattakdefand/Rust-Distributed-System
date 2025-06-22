mod optimizer;

use axum::{
    routing::{post},
    Json, Router, response::IntoResponse,
};
use optimizer::{DeploymentIntent, optimize};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/optimize", post(optimize_handler));

    println!("Multi-Objective Optimizer running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /optimize
async fn optimize_handler(Json(intent): Json<DeploymentIntent>) -> impl IntoResponse {
    let decision = optimizer::optimize(&intent);
    Json(decision)
}
