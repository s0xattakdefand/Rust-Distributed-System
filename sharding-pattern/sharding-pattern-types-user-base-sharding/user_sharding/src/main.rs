use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    shard_id: usize,
}

type ShardStorage = Arc<Mutex<HashMap<usize, Vec<User>>>>;

#[tokio::main]
async fn main() {
    let shard_storage: ShardStorage = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/add_user/:id/:name", get(add_user))
        .route("/get_users/:shard_id", get(get_users))
        .with_state(shard_storage.clone());

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn add_user(
    State(shard_storage): State<ShardStorage>,
    Path((id, name)): Path<(String, String)>,
) -> Json<String> {
    let shard_id = shard_for_user(&id);
    let user = User {
        id,
        name,
        shard_id,
    };

    let mut storage = shard_storage.lock().await;
    storage.entry(shard_id).or_default().push(user.clone());

    Json(format!("✅ Added user {:?} to shard {}", user.name, shard_id))
}

async fn get_users(
    State(shard_storage): State<ShardStorage>,
    Path(shard_id): Path<usize>,
) -> Json<Vec<User>> {
    let storage = shard_storage.lock().await;
    let users = storage.get(&shard_id).cloned().unwrap_or_default();
    Json(users)
}

fn shard_for_user(user_id: &str) -> usize {
    let hash = seahash::hash(user_id.as_bytes());
    (hash % 4) as usize // 4 shards for example
}
