use crate::store::SharedStore;
use tokio::time::{sleep, Duration};

pub fn spawn_sync_task(store: SharedStore) {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            let mut lock = store.lock().await;
            if !lock.pending.is_empty() {
                println!("🔁 Syncing {} pending user(s)...", lock.pending.len());
                lock.flush_pending();
            }
        }
    });
}
