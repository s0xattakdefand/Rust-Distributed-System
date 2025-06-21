mod society;

use axum::{
    routing::{post, get},
    Json, Router, response::{IntoResponse, Response},
};
use society::{SocietyContext, ExternalSignal};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber;

static SOCIETY: Lazy<Mutex<SocietyContext>> = Lazy::new(|| Mutex::new(SocietyContext::new()));

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/external_signal", post(external_signal))
        .route("/call_api", get(call_api))
        .route("/status", get(status));

    println!("Societal Feedback Loops running at http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// POST /external_signal  { "signal": "grid_overload" }
async fn external_signal(Json(sig): Json<ExternalSignal>) -> impl IntoResponse {
    let mut ctx = SOCIETY.lock().unwrap();
    ctx.process_signal(sig);
    Json(ctx.get_status())
}

// GET /call_api
async fn call_api() -> Response {
    let ctx = SOCIETY.lock().unwrap();
    if ctx.maintenance_mode {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            ctx.reason.clone().unwrap_or("API in maintenance mode.".to_string()),
        )
            .into_response()
    } else if ctx.throttled {
        (
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            ctx.reason.clone().unwrap_or("API is being throttled.".to_string()),
        )
            .into_response()
    } else {
        (
            axum::http::StatusCode::OK,
            "API call succeeded (no societal constraints)".to_string(),
        )
            .into_response()
    }
}

// GET /status
async fn status() -> impl IntoResponse {
    let ctx = SOCIETY.lock().unwrap();
    Json(ctx.get_status())
}
