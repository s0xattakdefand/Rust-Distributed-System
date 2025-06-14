use axum::{Router, routing::get, Json};
use serde_json::json;

pub fn app_routes() -> Router {
    Router::new().route("/data", get(secured_handler))
}

async fn secured_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "✅ Access granted by OPA!"
    }))
}
