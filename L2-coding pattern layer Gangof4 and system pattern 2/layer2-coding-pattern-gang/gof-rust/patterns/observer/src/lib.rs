//! Observer with weak refs.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub trait Observer {
    fn update(&self, v: i32);
}

pub struct Subject {
    value: i32,
    observers: RefCell<Vec<Weak<dyn Observer>>>,
}
impl Subject {
    pub fn new() -> Self {
        Self {
            value: 0,
            observers: RefCell::new(Vec::new()),
        }
    }
    pub fn attach(&self, o: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(Rc::downgrade(&o));
    }
    pub fn set(&mut self, v: i32) {
        self.value = v;
        self.notify();
    }
    fn notify(&self) {
        self.observers.borrow_mut().retain(|w| {
            if let Some(o) = w.upgrade() {
                o.update(self.value);
                true
            } else {
                false
            }
        });
    }
}
impl Default for Subject {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PrintObserver;
impl Observer for PrintObserver {
    fn update(&self, v: i32) {
        println!("value={v}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn notify() {
        let mut s = Subject::default();
        let o = Rc::new(PrintObserver);
        s.attach(o);
        s.set(7);
    }
}
