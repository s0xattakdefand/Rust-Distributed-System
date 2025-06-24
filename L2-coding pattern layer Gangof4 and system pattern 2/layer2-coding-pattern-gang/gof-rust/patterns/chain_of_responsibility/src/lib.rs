//! Logger chain.

#[derive(PartialEq, PartialOrd)]
pub enum Level {
    Info,
    Warn,
    Error,
}

pub trait Handler {
    fn set_next(&mut self, next: Option<Box<dyn Handler>>);
    fn handle(&self, lvl: Level, msg: &str);
}

macro_rules! impl_handler {
    ($ty:ident, $min:expr) => {
        pub struct $ty {
            next: Option<Box<dyn Handler>>,
        }
        impl $ty {
            pub fn new() -> Self {
                Self { next: None }
            }
        }
        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Handler for $ty {
            fn set_next(&mut self, next: Option<Box<dyn Handler>>) {
                self.next = next;
            }
            fn handle(&self, lvl: Level, msg: &str) {
                if lvl >= $min {
                    println!(concat!(stringify!($ty), ": {}"), msg);
                }
                if let Some(ref n) = self.next {
                    n.handle(lvl, msg);
                }
            }
        }
    };
}
impl_handler!(ConsoleLogger, Level::Info);
impl_handler!(EmailLogger, Level::Warn);
impl_handler!(SmsLogger, Level::Error);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chain() {
        let mut console = ConsoleLogger::new();
        let mut email = EmailLogger::new();
        let sms = SmsLogger::new();

        email.set_next(Some(Box::new(sms)));
        console.set_next(Some(Box::new(email)));

        console.handle(Level::Error, "disk down");
    }
}
