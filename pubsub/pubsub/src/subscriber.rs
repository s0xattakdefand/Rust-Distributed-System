use axum::{extract::State, response::IntoResponse};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use crate::message::Message;
use tokio::time::{sleep, Duration};

pub async fn subscribe(
    State(tx): State<Arc<Mutex<Sender<Message>>>>,
) -> impl IntoResponse {
    let mut rx = tx.lock().unwrap().subscribe();

    tokio::spawn(async move {
        for _ in 0..5 {
            match rx.recv().await {
                Ok(msg) => println!("📩 Subscriber received: {:?}", msg),
                Err(e) => println!("⚠️ Error receiving message: {}", e),
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    "👂 Subscribed for 5 messages"
}