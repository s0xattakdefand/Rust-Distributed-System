mod auth;
mod middleware;
mod routes;
mod client;

use axum::Router;
use axum::middleware::from_fn;
use middleware::token_validator;
use routes::routes;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use dotenv::dotenv;
use tokio::net::TcpListener;
use axum::serve; // ✅ Axum's built-in server wrapper

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .merge(routes())
        .layer(from_fn(token_validator))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap(); // ✅ no more hyper::Server
}
