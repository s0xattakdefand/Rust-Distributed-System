use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct HeartbeatManager {
    pub nodes: Arc<Mutex<HashMap<String, Instant>>>,
    pub timeout: Duration,
}

impl HeartbeatManager {
    pub fn new(timeout_secs: u64) -> Self {
        HeartbeatManager {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn receive_heartbeat(&self, node_id: String) {
        let mut nodes = self.nodes.lock().await;
        nodes.insert(node_id, Instant::now());
    }

    pub async fn get_active_nodes(&self) -> Vec<String> {
        let nodes = self.nodes.lock().await;
        let now = Instant::now();
        nodes
            .iter()
            .filter(|&(_, time)| now.duration_since(*time) < self.timeout)
            .map(|(id, _)| id.clone())
            .collect()
    }
}
