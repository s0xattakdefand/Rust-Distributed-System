//! Simple deduper using HashSet of ids R<T>.

use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Default)]
pub struct Deduper { seen: Mutex<HashSet<String>> }

impl Deduper {
    pub fn accept(&self, id: &str) -> bool {
        let mut s = self.seen.lock().unwrap();
        s.insert(id.into()) // true if newly inserted
    }
}

#[cfg(test)] mod tests{use super::*;#[test]fn dup(){let d=Deduper::default();assert!(d.accept("A"));assert!(!d.accept("A"));}}
