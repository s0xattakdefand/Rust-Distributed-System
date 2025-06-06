pub fn get_shard(user_id: &str, total_shards: usize) -> usize {
    let hash = seahash::hash(user_id.as_bytes());
    (hash % total_shards as u64) as usize
}
