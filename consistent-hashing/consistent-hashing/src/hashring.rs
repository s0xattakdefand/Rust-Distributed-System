use crate::node::Node;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub struct ConsistentHashRing {
    ring: BTreeMap<u64, Node>,
    replicas: usize,
}

impl ConsistentHashRing {
    pub fn new(replicas: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            replicas,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        for i in 0..self.replicas {
            let key = self.hash(&format!("{}-{}", node.id, i));
            self.ring.insert(key, node.clone());
        }
    }

    pub fn remove_node(&mut self, node: &Node) {
        for i in 0..self.replicas {
            let key = self.hash(&format!("{}-{}", node.id, i));
            self.ring.remove(&key);
        }
    }

    pub fn get_node(&self, item_key: &str) -> Option<&Node> {
        if self.ring.is_empty() {
            return None;
        }

        let key = self.hash(item_key);

        // Find the first node with a hash >= key (or wrap around)
        self.ring
            .range(key..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, node)| node)
    }

    pub fn all_nodes(&self) -> Vec<Node> {
        let mut seen = std::collections::HashSet::new();
        self.ring.values().cloned().filter(|n| seen.insert(n.id.clone())).collect()
    }

    fn hash(&self, input: &str) -> u64 {
        let digest = Sha256::digest(input.as_bytes());
        u64::from_be_bytes([
            digest[0], digest[1], digest[2], digest[3],
            digest[4], digest[5], digest[6], digest[7],
        ])
    }
}
