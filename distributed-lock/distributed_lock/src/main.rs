use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct Lock {
    owner: String,
}

type LockTable = Arc<Mutex<HashMap<String, Lock>>>;

#[tokio::main]
async fn main() {
    let locks: LockTable = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/lock/:resource", post(acquire_lock))
        .route("/unlock/:resource/:owner", post(release_lock))
        .with_state(locks);

    let addr: SocketAddr = "127.0.0.1:4001".parse().unwrap();
    println!("🔐 Distributed Lock Server running at {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn acquire_lock(
    Path(resource): Path<String>,
    State(lock_table): State<LockTable>,
) -> Json<String> {
    let mut locks = lock_table.lock().await;
    if locks.contains_key(&resource) {
        Json("❌ Resource is already locked".into())
    } else {
        let owner = Uuid::new_v4().to_string();
        locks.insert(resource.clone(), Lock { owner: owner.clone() });
        Json(format!("✅ Lock acquired by {}", owner))
    }
}

async fn release_lock(
    Path((resource, owner)): Path<(String, String)>,
    State(lock_table): State<LockTable>,
) -> Json<String> {
    let mut locks = lock_table.lock().await;
    match locks.get(&resource) {
        Some(lock) if lock.owner == owner => {
            locks.remove(&resource);
            Json("🔓 Lock released".into())
        }
        _ => Json("🚫 Unauthorized or no lock found".into()),
    }
}
