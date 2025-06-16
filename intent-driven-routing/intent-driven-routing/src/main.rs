mod intent;
mod router;
mod services;

use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    response::IntoResponse,
};
use intent::Intent;

#[derive(Clone)]
struct AppState;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/route", post(route_handler))
        .with_state(AppState);

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn route_handler(
    State(_): State<AppState>,
    Json(intent): Json<Intent>
) -> impl IntoResponse {
    let decision = router::pick_route(&intent);
    Json(serde_json::json!({
        "intent": intent,
        "decision": decision,
    }))
}
