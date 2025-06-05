mod hashring;
mod node;

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use hashring::ConsistentHashRing;
use node::Node;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

type SharedRing = Arc<Mutex<ConsistentHashRing>>;

#[tokio::main]
async fn main() {
    let ring = Arc::new(Mutex::new(ConsistentHashRing::new(100)));

    let app = Router::new()
        .route("/add/:id", post({
            let ring = ring.clone();
            move |Path(id): Path<String>| {
                let ring = ring.clone();
                async move {
                    ring.lock().unwrap().add_node(Node { id: id.clone() });
                    Json(format!("✅ Node '{}' added.", id))
                }
            }
        }))
        .route("/lookup/:key", get({
            let ring = ring.clone();
            move |Path(key): Path<String>| {
                let ring = ring.clone();
                async move {
                    let guard = ring.lock().unwrap();         // ✅ Hold the lock
                    let result = guard.get_node(&key);        // ✅ Borrow from guard safely

                    match result {
                        Some(node) => Json(format!("🔑 '{}' → 🧩 Node '{}'", key, node.id)),
                        None => Json("🚫 No nodes in ring.".to_string()),
                    }
                }
            }
        }))
        .route("/nodes", get({
            let ring = ring.clone();
            move || {
                let ring = ring.clone();
                async move {
                    let guard = ring.lock().unwrap();
                    let nodes = guard.all_nodes();
                    Json(nodes)
                }
            }
        }))
        .route("/remove/:id", post({
            let ring = ring.clone();
            move |Path(id): Path<String>| {
                let ring = ring.clone();
                async move {
                    ring.lock().unwrap().remove_node(&Node { id: id.clone() });
                    Json(format!("❌ Node '{}' removed.", id))
                }
            }
        }));

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap(); // ✅ Fix for E0284
    println!("🌀 Consistent Hashing server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
