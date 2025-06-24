// TODO: src
//! Thread-safe global Logger using OnceLock (stable).

use std::sync::OnceLock;

pub struct Logger;
impl Logger {
    pub fn log(&self, msg: &str) { println!("[LOG] {msg}"); }
}

static INSTANCE: OnceLock<Logger> = OnceLock::new();

pub fn global() -> &'static Logger {
    INSTANCE.get_or_init(|| Logger)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn single_instance() {
        let a = global() as *const _;
        let b = global() as *const _;
        assert_eq!(a, b); // same address
    }
}
