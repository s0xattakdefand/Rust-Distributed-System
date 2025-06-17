mod intent;
mod orchestrator;
mod actions;

use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    response::IntoResponse,
};
use intent::Intent;
use orchestrator::orchestrate;
use tracing_subscriber;

#[derive(Clone)]
struct AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/orchestrate", post(orchestrate_handler))
        .with_state(AppState);

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn orchestrate_handler(
    State(_): State<AppState>,
    Json(intent): Json<Intent>
) -> impl IntoResponse {
    let steps = orchestrate(&intent);
    Json(serde_json::json!({
        "intent": intent,
        "workflow": steps,
    }))
}
