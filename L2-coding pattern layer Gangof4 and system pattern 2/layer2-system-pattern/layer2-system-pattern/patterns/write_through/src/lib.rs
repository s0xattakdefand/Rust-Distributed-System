//! Write-Through cache: write to cache **and** enqueue DB flush.

use cache_aside::{Cache, Db};
use tokio::sync::mpsc;

/// A thin wrapper that writes to cache immediately and
/// pushes the key/value pair onto an async channel for eventual DB flush.
pub struct WriteThrough<C, D> {
    cache: C,
    tx:    mpsc::Sender<(String, String)>,
    _db:   std::marker::PhantomData<D>,
}

impl<C, D> WriteThrough<C, D>
where
    C: Cache + Send + Sync + 'static,
    D: Db + Send + Sync + 'static,
{
    pub fn new(cache: C, db: D) -> Self {
        let (tx, mut rx) = mpsc::channel::<(String, String)>(32);

        // Spawn background task that “writes” to DB.
        tokio::spawn(async move {
            while let Some((k, v)) = rx.recv().await {
                // In a real impl: db.upsert(k, v).  Here we just call fetch for demo.
                let _ = db.fetch(&k);
                println!("flushed {k}={v} to DB");
            }
        });

        Self {
            cache,
            tx,
            _db: std::marker::PhantomData,
        }
    }

    /// Write value; never blocks on DB.
    pub async fn write(&self, key: &str, val: &str) {
        self.cache.set(key, val);
        let _ = self.tx.send((key.into(), val.into())).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache_aside::{Cache, Db};

    // Dumb in-mem cache & DB for tests
    struct C;
    impl Cache for C {
        fn get(&self, _k: &str) -> Option<String> {
            None
        }
        fn set(&self, _k: &str, _v: &str) {}
    }

    struct D;
    impl Db for D {
        fn fetch(&self, _k: &str) -> Option<String> {
            None
        }
    }

    #[tokio::test]
    async fn write_doesnt_panic() {
        let wt = WriteThrough::new(C, D);
        wt.write("key", "val").await; // should compile & run
    }
}
