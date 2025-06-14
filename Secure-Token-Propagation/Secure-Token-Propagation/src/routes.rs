use axum::{Json, Router, routing::get};
use serde_json::json;
use crate::auth::create_token;

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/service-a", get(service_a))
}

async fn login_handler() -> Json<serde_json::Value> {
    let token = create_token("user123");
    Json(json!({ "access_token": token }))
}

async fn service_a() -> Json<serde_json::Value> {
    Json(json!({
        "status": "service-a received request with valid token"
    }))
}
