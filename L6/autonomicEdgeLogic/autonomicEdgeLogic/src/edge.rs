use std::collections::{HashMap, VecDeque};
use rand::Rng;

#[derive(Debug, Clone)]
pub enum Decision {
    CacheHit,
    CacheMissAndFetched,
    Blocked,
    Rerouted,
}

#[derive(Debug)]
pub struct EdgeNode {
    pub cache: HashMap<String, String>,
    pub cache_history: VecDeque<String>,
    pub cache_size: usize,
    pub blocked_ips: Vec<String>,
}

impl EdgeNode {
    pub fn new(cache_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            cache_history: VecDeque::with_capacity(cache_size),
            cache_size,
            blocked_ips: Vec::new(),
        }
    }

    /// Decide what to do with the request: cache, block, or reroute
    pub fn handle_request(&mut self, ip: &str, key: &str) -> Decision {
        // Security logic: block known-bad IPs
        if self.blocked_ips.contains(&ip.to_string()) {
            return Decision::Blocked;
        }

        // Randomly simulate backend trouble, so we decide to reroute
        if rand::thread_rng().gen_bool(0.05) {
            return Decision::Rerouted;
        }

        // Local caching logic
        if let Some(_) = self.cache.get(key) {
            return Decision::CacheHit;
        } else {
            // Simulate fetching from backend
            let value = format!("data_for_{}", key);
            self.cache.insert(key.to_string(), value);

            // Track LRU
            self.cache_history.push_back(key.to_string());
            if self.cache_history.len() > self.cache_size {
                if let Some(evict) = self.cache_history.pop_front() {
                    self.cache.remove(&evict);
                }
            }
            return Decision::CacheMissAndFetched;
        }
    }

    pub fn block_ip(&mut self, ip: String) {
        if !self.blocked_ips.contains(&ip) {
            self.blocked_ips.push(ip);
        }
    }
}
