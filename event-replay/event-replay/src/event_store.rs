use std::collections::HashMap;
use crate::event::AccountEvent;
use crate::model::Account;

pub struct EventStore {
    store: HashMap<String, Vec<AccountEvent>>,
}

impl EventStore {
    pub fn new() -> Self {
        EventStore {
            store: HashMap::new(),
        }
    }

    pub fn append(&mut self, user_id: &str, event: AccountEvent) {
        self.store.entry(user_id.to_string())
            .or_default()
            .push(event);
    }

    pub fn replay(&self, user_id: &str) -> Option<Account> {
        let events = self.store.get(user_id)?;
        let mut account = Account {
            user_id: user_id.to_string(),
            balance: 0,
        };
        for event in events {
            match event {
                AccountEvent::Deposited(amount) => account.balance += amount,
                AccountEvent::Withdrawn(amount) => account.balance -= amount,
            }
        }
        Some(account)
    }

    pub fn all_events(&self, user_id: &str) -> Option<&Vec<AccountEvent>> {
        self.store.get(user_id)
    }
}
