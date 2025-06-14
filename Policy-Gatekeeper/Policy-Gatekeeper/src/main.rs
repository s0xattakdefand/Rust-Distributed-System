mod opa;
mod middleware;
mod routes;

use axum::{Router, middleware::from_fn};
use middleware::opa_gatekeeper;
use routes::app_routes;
use std::net::SocketAddr;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .merge(app_routes())
        .layer(from_fn(opa_gatekeeper));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Policy Gatekeeper running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
