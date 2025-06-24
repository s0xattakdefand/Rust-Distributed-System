//! Token-bucket Redis rate-limiter stub.

pub trait RateLimiter { fn check(&self, key: &str) -> bool; }

pub struct LocalBucket {
    cap: usize,
    tokens: std::sync::Mutex<std::collections::HashMap<String,usize>>,
}
impl LocalBucket {
    pub fn new(cap:usize)->Self{Self{cap,tokens:Default::default()}}
}
impl RateLimiter for LocalBucket {
    fn check(&self, key:&str)->bool{
        let mut m=self.tokens.lock().unwrap();
        let e=m.entry(key.into()).or_insert(self.cap);
        if *e>0 { *e-=1; true } else { false }
    }
}
#[cfg(test)]mod tests{use super::*;#[test]fn bucket(){let b=LocalBucket::new(1);assert!(b.check("k"));assert!(!b.check("k"));}}
