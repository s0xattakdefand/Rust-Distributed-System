use crate::shard::Shard;

#[derive(Debug)]
pub struct Sharder {
    shards: Vec<Shard>,
}

impl Sharder {
    pub fn new(shard_count: usize) -> Self {
        let shards = (0..shard_count).map(Shard::new).collect();
        Self { shards }
    }

    fn get_shard_index(&self, key: &str) -> usize {
        let hash = fxhash::hash32(key.as_bytes());
        (hash as usize) % self.shards.len()
    }

    pub fn store(&mut self, key: &str, value: String) {
        let idx = self.get_shard_index(key);
        self.shards[idx].store(value);
    }

    pub fn get_all(&self) -> Vec<Shard> {
        self.shards.clone()
    }
}
