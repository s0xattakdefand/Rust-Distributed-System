use std::collections::HashMap;
use crate::user::User;

pub struct Shard {
    pub id: u64,
    pub users: HashMap<u64, User>,
}

impl Shard {
    pub fn new(id: u64) -> Self {
        Shard {
            id,
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }
}
