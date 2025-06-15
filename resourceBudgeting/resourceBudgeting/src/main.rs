mod budget;
mod metrics;

use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
    http::StatusCode,
};
use std::sync::Arc;
use budget::ResourceBudget;
use metrics::{REQUESTS, REJECTED, render_metrics};

#[derive(Clone)]
struct AppState {
    budget: Arc<ResourceBudget>,
}

#[tokio::main]
async fn main() {
    // Set a resource budget of 100 allowed requests
    let budget = Arc::new(ResourceBudget::new(100));
    let state = AppState { budget };

    let app = Router::new()
        .route("/", get(handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    println!("Listening on http://127.0.0.1:3000/");
    // Use axum::serve in 0.7+
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app
    )
    .await
    .unwrap();
}

async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.budget.increment() {
        REQUESTS.inc();
        (
            StatusCode::OK,
            format!("Request OK. Remaining: {}", state.budget.remaining())
        )
    } else {
        REJECTED.inc();
        (
            StatusCode::TOO_MANY_REQUESTS,
            format!("Error budget exceeded! Try again later."),
        )
    }
}

async fn metrics_handler() -> impl IntoResponse {
    render_metrics()
}
