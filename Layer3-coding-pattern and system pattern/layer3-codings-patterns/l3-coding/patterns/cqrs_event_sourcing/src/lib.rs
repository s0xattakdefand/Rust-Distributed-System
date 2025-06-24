//! CQRS + Event Sourcing (in-memory).

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event { Credited(u32), Debited(u32) }

#[derive(Default)]
pub struct Store { log: Vec<Event> }

impl Store {
    pub fn append(&mut self, e: Event) { self.log.push(e); }
    pub fn balance(&self) -> i32 {
        self.log.iter().fold(0, |acc, e| match e {
            Event::Credited(n) => acc + *n as i32,
            Event::Debited(n)  => acc - *n as i32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay() {
        let mut s = Store::default();
        s.append(Event::Credited(100));
        s.append(Event::Debited(60));
        assert_eq!(s.balance(), 40);
    }
}
