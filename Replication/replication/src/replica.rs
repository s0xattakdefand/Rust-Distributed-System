use std::collections::HashMap;
use crate::types::Data;

#[derive(Debug)]
pub struct ReplicaSet {
    pub replicas: Vec<HashMap<String, String>>,
}

impl ReplicaSet {
    pub fn new(count: usize) -> Self {
        let replicas = vec![HashMap::new(); count];
        Self { replicas }
    }

    pub fn replicate(&mut self, data: Data) {
        for (i, replica) in self.replicas.iter_mut().enumerate() {
            replica.insert(data.key.clone(), data.value.clone());
            println!("Replica {} stored: {:?} = {:?}", i, data.key, data.value);
        }
    }
}
