//! Stub trait mirroring Consul-like API; unit-test proves compile.

pub trait Registry {
    fn register(&self, name: &str, url: &str);
    fn discover(&self, name: &str) -> Option<String>;
}

// dummy in-memory impl for tests
use std::collections::HashMap;
#[derive(Default)]
pub struct MemRegistry { map: std::sync::Mutex<HashMap<String,String>> }
impl Registry for MemRegistry {
    fn register(&self, name:&str, url:&str){ self.map.lock().unwrap().insert(name.into(),url.into()); }
    fn discover(&self, name:&str)->Option<String>{ self.map.lock().unwrap().get(name).cloned() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn register_and_lookup() {
        let r = MemRegistry::default();
        r.register("svc","http://x");
        assert_eq!(r.discover("svc").unwrap(),"http://x");
    }
}
