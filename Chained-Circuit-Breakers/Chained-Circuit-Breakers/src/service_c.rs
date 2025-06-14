use axum::{Router, routing::get, Json};
use serde_json::json;

pub fn service_c_router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Json<serde_json::Value> {
    Json(json!({ "service": "C", "status": "fast!" }))
}
