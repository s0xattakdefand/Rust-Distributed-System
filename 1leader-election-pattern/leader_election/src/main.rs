mod election;
mod node;
mod utils;

use election::LeaderElection;
use std::sync::Arc;
use tokio::sync::Mutex;
use utils::launch_web_server;

#[tokio::main]
async fn main() {
    let node_id = utils::generate_node_id();
    let election = Arc::new(Mutex::new(LeaderElection::new(node_id)));

    let cloned = election.clone();
    tokio::spawn(async move {
        cloned.lock().await.start().await;
    });

    launch_web_server(election).await;
}
