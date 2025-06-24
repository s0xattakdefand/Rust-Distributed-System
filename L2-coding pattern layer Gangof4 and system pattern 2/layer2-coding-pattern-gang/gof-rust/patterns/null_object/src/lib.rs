// TODO: src
//! No-op logger implements same interface.

pub trait Logger { fn log(&self, _: &str); }

pub struct RealLogger;
impl Logger for RealLogger {
    fn log(&self, msg:&str) { println!("LOG: {msg}"); }
}

pub struct NullLogger;
impl Logger for NullLogger { fn log(&self, _: &str) { /* nothing */ } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn silent() {
        let l: &dyn Logger = &NullLogger;
        l.log("ignored");
    }
}
