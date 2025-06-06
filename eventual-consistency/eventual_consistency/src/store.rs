use crate::model::User;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

pub type SharedStore = Arc<Mutex<InMemoryStore>>;

#[derive(Default)]
pub struct InMemoryStore {
    pub pending: Vec<User>,
    pub stable: HashMap<String, User>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            stable: HashMap::new(),
        }
    }

    pub fn add_to_pending(&mut self, user: User) {
        self.pending.push(user);
    }

    pub fn flush_pending(&mut self) {
        for user in self.pending.drain(..) {
            self.stable.insert(user.id.clone(), user);
        }
    }
}
