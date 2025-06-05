mod api;
mod model;
mod shard;

use axum::{routing::get, Router};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use api::{add_user, get_users, ShardStorage};

#[tokio::main]
async fn main() {
    let storage: ShardStorage = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/add_user/:id/:name", get(add_user))
        .route("/get_users/:shard_id", get(get_users))
        .with_state(storage);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("🚀 Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
