use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ResourceBudget {
    pub limit: usize,
    pub used: AtomicUsize,
}

impl ResourceBudget {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            used: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) -> bool {
        let prev = self.used.fetch_add(1, Ordering::SeqCst);
        prev + 1 <= self.limit
    }

    pub fn remaining(&self) -> usize {
        self.limit.saturating_sub(self.used.load(Ordering::SeqCst))
    }
}
