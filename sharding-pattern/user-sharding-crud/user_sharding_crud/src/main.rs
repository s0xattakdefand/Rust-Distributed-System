mod api;
mod model;
mod shard;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use api::{add_user, get_user, update_user, delete_user, get_users, ShardStorage};

#[tokio::main]
async fn main() {
    // Shared storage for user sharding
    let storage: ShardStorage = Arc::new(Mutex::new(HashMap::new()));

    // Main router with static file serving (index.html)
    let app = Router::new()
        // Static HTML frontend
        .nest_service("/", ServeDir::new("."))

        // CRUD routes
        .route("/add_user/:id/:name", post(add_user))
        .route("/get_user/:id", get(get_user))
        .route("/update_user/:id/:name", put(update_user))
        .route("/delete_user/:id", delete(delete_user))
        .route("/get_users/:shard_id", get(get_users))

        // Pass shared state (sharded storage)
        .with_state(storage);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🚀 User-Based Sharding API on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
