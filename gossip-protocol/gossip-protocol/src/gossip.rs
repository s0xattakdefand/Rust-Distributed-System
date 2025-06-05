use crate::types::{Node, GossipMessage};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;

pub struct GossipState {
    pub nodes: Arc<Mutex<HashMap<String, Node>>>,
    pub self_node: Node,
    pub client: Client,
}

impl GossipState {
    pub fn new(id: String, address: String) -> Self {
        let now = current_ts();
        let self_node = Node {
            id: id.clone(),
            address: address.clone(),
            last_seen: now,
        };

        let mut map = HashMap::new();
        map.insert(id.clone(), self_node.clone());

        GossipState {
            nodes: Arc::new(Mutex::new(map)),
            self_node,
            client: Client::new(),
        }
    }

    pub async fn gossip(&self) {
        let all_nodes = self.nodes.lock().await;
        let peers: Vec<_> = all_nodes
            .values()
            .filter(|n| n.id != self.self_node.id)
            .cloned()
            .collect();

        let peers_to_contact = peers.choose_multiple(&mut rand::thread_rng(), 2);

        let msg = GossipMessage {
            from: self.self_node.id.clone(),
            known_nodes: all_nodes.values().cloned().collect(),
        };

        drop(all_nodes); // release lock early

        for peer in peers_to_contact {
            let url = format!("http://{}/gossip", peer.address);
            let _ = self.client
                .post(&url)
                .json(&msg) // ✅ this now works because of the correct feature
                .send()
                .await;
        }
    }

    pub async fn receive_gossip(&self, msg: GossipMessage) {
        let mut nodes = self.nodes.lock().await;
        let now = current_ts();

        for node in msg.known_nodes {
            nodes
                .entry(node.id.clone())
                .and_modify(|n| {
                    if node.last_seen > n.last_seen {
                        *n = node.clone();
                    }
                })
                .or_insert(Node {
                    id: node.id,
                    address: node.address,
                    last_seen: now,
                });
        }
    }

    pub async fn get_nodes(&self) -> Vec<Node> {
        self.nodes.lock().await.values().cloned().collect()
    }
}

fn current_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
