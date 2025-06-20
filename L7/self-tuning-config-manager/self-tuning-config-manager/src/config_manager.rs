use std::collections::{HashMap, VecDeque};
use rand::Rng;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CacheStats {
    pub hit_count: u32,
    pub miss_count: u32,
    pub hit_rate: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ConfigState {
    pub cache_size: usize,
    pub stats: CacheStats,
}

pub struct SelfTuningConfigManager {
    cache: HashMap<String, String>,
    lru: VecDeque<String>,
    pub cache_size: usize,
    pub hit_count: u32,
    pub miss_count: u32,
}

impl SelfTuningConfigManager {
    pub fn new(initial_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            lru: VecDeque::with_capacity(initial_size),
            cache_size: initial_size,
            hit_count: 0,
            miss_count: 0,
        }
    }

    pub fn access(&mut self, key: &str) {
        if self.cache.contains_key(key) {
            self.hit_count += 1;
            self.lru.retain(|k| k != key);
            self.lru.push_back(key.to_string());
        } else {
            self.miss_count += 1;
            self.cache.insert(key.to_string(), format!("data_{}", key));
            self.lru.push_back(key.to_string());
            if self.lru.len() > self.cache_size {
                if let Some(evict) = self.lru.pop_front() {
                    self.cache.remove(&evict);
                }
            }
        }
    }

    pub fn feedback_tune(&mut self) {
        let total = self.hit_count + self.miss_count;
        let hit_rate = if total > 0 {
            self.hit_count as f32 / total as f32
        } else {
            1.0
        };

        // Auto-tune logic
        if hit_rate < 0.7 && self.cache_size < 100 {
            self.cache_size += 1; // increase cache size if too many misses
        } else if hit_rate > 0.9 && self.cache_size > 5 {
            self.cache_size -= 1; // shrink cache if memory can be saved
        }

        // Reset stats for next round
        self.hit_count = 0;
        self.miss_count = 0;
    }

    pub fn get_state(&self) -> ConfigState {
        let total = self.hit_count + self.miss_count;
        let hit_rate = if total > 0 {
            self.hit_count as f32 / total as f32
        } else {
            1.0
        };

        ConfigState {
            cache_size: self.cache_size,
            stats: CacheStats {
                hit_count: self.hit_count,
                miss_count: self.miss_count,
                hit_rate,
            },
        }
    }
}
