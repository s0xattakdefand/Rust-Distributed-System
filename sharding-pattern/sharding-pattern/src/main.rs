mod sharder;
mod shard;

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use sharder::Sharder;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

type SharedSharder = Arc<Mutex<Sharder>>;

#[tokio::main]
async fn main() {
    let sharder = Arc::new(Mutex::new(Sharder::new(4)));

    let app = Router::new()
        .route("/store/:key/:value", post({
            let sharder = sharder.clone();
            move |Path((key, value)): Path<(String, String)>| {
                let sharder = sharder.clone();
                async move {
                    sharder.lock().unwrap().store(&key, value.clone());
                    Json(format!("Stored '{}' for key '{}'", value, key))
                }
            }
        }))
        .route("/dump", get({
            let sharder = sharder.clone();
            move || {
                let sharder = sharder.clone();
                async move {
                    let data = sharder.lock().unwrap().get_all();
                    Json(data
                        .into_iter()
                        .map(|shard| format!("Shard {}: {:?}", shard.id, shard.data))
                        .collect::<Vec<_>>())
                }
            }
        }));

    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
