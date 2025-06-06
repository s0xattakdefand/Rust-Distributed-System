use axum::{extract::State, Json};
use crate::store::SharedStore;
use crate::model::User;
use uuid::Uuid;

pub async fn create_user(
    State(store): State<SharedStore>,
    Json(payload): Json<User>,
) -> Json<String> {
    let mut lock = store.lock().await;
    let mut user = payload;
    user.id = Uuid::new_v4().to_string();
    lock.add_to_pending(user.clone());
    Json(format!("🕐 User '{}' queued for eventual write", user.name))
}
