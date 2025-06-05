mod gossip;
mod types;

use axum::{Json, Router, routing::{get, post}};
use gossip::GossipState;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{self, Duration};
use types::GossipMessage;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let my_id = args.get(1).expect("Node ID required").clone();
    let my_addr = args.get(2).expect("Address required").clone(); // e.g. "127.0.0.1:3000"

    let state = Arc::new(GossipState::new(my_id.clone(), my_addr.clone()));

    // Start gossip loop
    let gossiper = state.clone();
    tokio::spawn(async move {
        loop {
            gossiper.gossip().await;
            time::sleep(Duration::from_secs(5)).await;
        }
    });

    // HTTP API
    let app = Router::new()
        .route("/gossip", post({
            let state = state.clone();
            move |Json(msg): Json<GossipMessage>| {
                let state = state.clone();
                async move {
                    state.receive_gossip(msg).await;
                    Json("ok")
                }
            }
        }))
        .route("/nodes", get({
            let state = state.clone();
            move || {
                let state = state.clone();
                async move {
                    Json(state.get_nodes().await)
                }
            }
        }));

    let addr: SocketAddr = my_addr.parse().unwrap();
    println!("🗣️  Gossip node running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
