use axum::{
    extract::{Path, State},
    Json,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{model::User, shard::get_shard};

pub type ShardStorage = Arc<Mutex<HashMap<usize, Vec<User>>>>;

// ➕ Create User
pub async fn add_user(
    State(storage): State<ShardStorage>,
    Path((id, name)): Path<(String, String)>,
) -> Json<String> {
    let shard_id = get_shard(&id, 4);
    let user = User { id, name, shard_id };

    let mut locked = storage.lock().await;
    locked.entry(shard_id).or_default().push(user.clone());

    Json(format!("✅ Added user '{}' to shard {}", user.name, shard_id))
}

// 🔍 Read User
pub async fn get_user(
    State(storage): State<ShardStorage>,
    Path(id): Path<String>,
) -> Json<String> {
    let shard_id = get_shard(&id, 4);
    let locked = storage.lock().await;
    if let Some(users) = locked.get(&shard_id) {
        if let Some(user) = users.iter().find(|u| u.id == id) {
            return Json(format!("👤 {} found in shard {} → name: {}", id, shard_id, user.name));
        }
    }
    Json("🚫 User not found".to_string())
}

// ✏️ Update User
pub async fn update_user(
    State(storage): State<ShardStorage>,
    Path((id, new_name)): Path<(String, String)>,
) -> Json<String> {
    let shard_id = get_shard(&id, 4);
    let mut locked = storage.lock().await;

    if let Some(users) = locked.get_mut(&shard_id) {
        if let Some(user) = users.iter_mut().find(|u| u.id == id) {
            user.name = new_name.clone();
            return Json(format!("✅ Updated user '{}' in shard {}", id, shard_id));
        }
    }

    Json("🚫 User not found".to_string())
}

// ❌ Delete User
pub async fn delete_user(
    State(storage): State<ShardStorage>,
    Path(id): Path<String>,
) -> Json<String> {
    let shard_id = get_shard(&id, 4);
    let mut locked = storage.lock().await;

    if let Some(users) = locked.get_mut(&shard_id) {
        let len_before = users.len();
        users.retain(|u| u.id != id);
        if users.len() < len_before {
            return Json(format!("🗑️ Deleted user '{}' from shard {}", id, shard_id));
        }
    }

    Json("🚫 User not found".to_string())
}

// 📦 Get All Users in Shard
pub async fn get_users(
    State(storage): State<ShardStorage>,
    Path(shard_id): Path<usize>,
) -> Json<Vec<User>> {
    let locked = storage.lock().await;
    Json(locked.get(&shard_id).cloned().unwrap_or_default())
}
