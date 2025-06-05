use axum::{
    extract::{Path, State},
    Json,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{model::User, shard::get_shard};

pub type ShardStorage = Arc<Mutex<HashMap<usize, Vec<User>>>>;

pub async fn add_user(
    State(storage): State<ShardStorage>,
    Path((id, name)): Path<(String, String)>,
) -> Json<String> {
    let shard_id = get_shard(&id, 4);
    let user = User { id, name, shard_id };

    let mut locked = storage.lock().await;
    locked.entry(shard_id).or_default().push(user.clone());

    Json(format!("✅ Added user {} to shard {}", user.name, shard_id))
}

pub async fn get_users(
    State(storage): State<ShardStorage>,
    Path(shard_id): Path<usize>,
) -> Json<Vec<User>> {
    let locked = storage.lock().await;
    let users = locked.get(&shard_id).cloned().unwrap_or_default();
    Json(users)
}
