mod breaker;
mod service_b;
mod service_c;

use axum::{Router, routing::get, extract::State, response::IntoResponse};
use std::sync::Arc;
use hyper::{Request, Response, body::Body};
use tower::util::BoxService;
use axum::body::to_bytes;
use serde_json::json;

type SharedService = Arc<BoxService<Request<Body>, Response<Body>, hyper::Error>>;

#[derive(Clone)]
struct AppState {
    svc_b: SharedService,
    svc_c: SharedService,
}

#[tokio::main]
async fn main() {
    let svc_b = Arc::new(breaker::build_breaker_stack("3001"));
    let svc_c = Arc::new(breaker::build_breaker_stack("3002"));

    let state = AppState { svc_b, svc_c };

    let router = Router::new()
        .route("/call-chain", get(call_chain))
        .with_state(state);

    tokio::spawn(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
        axum::serve(listener, service_b::service_b_router()).await.unwrap();
    });

    tokio::spawn(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
        axum::serve(listener, service_c::service_c_router()).await.unwrap();
    });

    println!("🚀 Server running at: http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(),
        router,
    )
    .await
    .unwrap();
}

async fn call_chain(State(state): State<AppState>) -> impl IntoResponse {
    let body_b = call_service(state.svc_b.clone()).await;
    let body_c = call_service(state.svc_c.clone()).await;

    axum::Json(json!({
        "service_b": body_b,
        "service_c": body_c
    }))
}

async fn call_service(svc: SharedService) -> String {
    let req = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    match svc.oneshot(req).await {
        Ok(resp) => {
            match to_bytes(resp.into_body(), usize::MAX).await {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(_) => "failed to read body".into(),
            }
        }
        Err(_) => "timeout or error".into(),
    }
}
