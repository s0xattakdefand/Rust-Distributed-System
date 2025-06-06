use axum::{extract::{State, Json}, response::IntoResponse};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use crate::message::Message;

pub async fn publish(
    State(tx): State<Arc<Mutex<Sender<Message>>>>,
    Json(msg): Json<Message>,
) -> impl IntoResponse {
    let tx = tx.lock().unwrap();
    match tx.send(msg.clone()) {
        Ok(_) => format!("✅ Published to topic: {}", msg.topic),
        Err(_) => "❌ Failed to publish".to_string(),
    }
}