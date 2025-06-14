use axum::{Router, routing::get, Json};
use serde_json::json;

pub fn service_b_router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Json<serde_json::Value> {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    Json(json!({ "service": "B", "status": "delayed!" }))
}
