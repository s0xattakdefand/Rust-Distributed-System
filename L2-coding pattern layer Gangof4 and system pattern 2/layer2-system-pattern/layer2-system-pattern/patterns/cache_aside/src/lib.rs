//! Cache-Aside helper with in-memory test stubs.

pub trait Db {
    fn fetch(&self, key: &str) -> Option<String>;
}

pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, val: &str);
}

/// Try cache → fallback DB → put loaded value into cache.
pub fn get_or_populate<C: Cache, D: Db>(
    cache: &C,
    db: &D,
    key: &str,
) -> Option<String> {
    if let Some(v) = cache.get(key) {
        return Some(v);
    }
    let v = db.fetch(key)?;
    cache.set(key, &v);
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    // dummy DB returns "db"
    struct MemDb;
    impl Db for MemDb {
        fn fetch(&self, _: &str) -> Option<String> {
            Some("db".into())
        }
    }
    // tiny cache storing a single optional value
    struct MemCache(std::cell::RefCell<Option<String>>);
    impl Cache for MemCache {
        fn get(&self, _k: &str) -> Option<String> {
            self.0.borrow().clone()
        }
        fn set(&self, _k: &str, v: &str) {
            *self.0.borrow_mut() = Some(v.into());
        }
    }

    #[test]
    fn first_db_then_cache() {
        let cache = MemCache(Default::default());
        let db = MemDb;

        // first call misses cache → DB
        assert_eq!(get_or_populate(&cache, &db, "k").unwrap(), "db");
        // second call hits cache
        assert_eq!(get_or_populate(&cache, &db, "k").unwrap(), "db");
    }
}
