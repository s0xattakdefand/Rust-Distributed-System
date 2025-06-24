//! In-memory event store + replay.

#[derive(Clone, Debug)]
pub enum LedgerEvent { Credit(f32), Debit(f32) }

#[derive(Default)]
pub struct EventStore { events: Vec<LedgerEvent> }

impl EventStore {
    pub fn append(&mut self, ev: LedgerEvent) { self.events.push(ev); }
    pub fn balance(&self) -> f32 {
        self.events.iter().fold(0.0, |acc,e| match e {
            LedgerEvent::Credit(v) => acc+v,
            LedgerEvent::Debit(v)  => acc-v,
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test] fn replay() {
        let mut es = EventStore::default();
        es.append(LedgerEvent::Credit(100.0));
        es.append(LedgerEvent::Debit(40.0));
        assert_eq!(es.balance(),60.0);
    }
}
