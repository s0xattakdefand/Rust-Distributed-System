use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    http::StatusCode,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthz", get(healthz_handler));

    println!("Listening on http://127.0.0.1:3000/");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn healthz_handler() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(String::from("OK"))  // <-- Use String instead of &str
        .unwrap()
}
