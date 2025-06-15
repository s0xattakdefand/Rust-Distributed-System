mod event;
mod event_store;
mod model;

use event::AccountEvent;
use event_store::EventStore;

fn main() {
    let user_id = "user1";
    let mut store = EventStore::new();

    // Simulate events
    store.append(user_id, AccountEvent::Deposited(500));
    store.append(user_id, AccountEvent::Withdrawn(100));
    store.append(user_id, AccountEvent::Deposited(300));

    // Replay account history
    let account = store.replay(user_id).unwrap();
    println!("Account after replay: {:?}", account);

    // List all events (log sourcing)
    println!("Event log for {}: {:?}", user_id, store.all_events(user_id).unwrap());
}
