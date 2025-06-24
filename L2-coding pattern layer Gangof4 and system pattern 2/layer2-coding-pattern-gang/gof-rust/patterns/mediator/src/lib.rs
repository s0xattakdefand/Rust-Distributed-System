// TODO: src
//! ChatRoom mediates User messages so they don’t know about each other.

use std::cell::RefCell;
use std::rc::{Rc,Weak};

pub struct ChatRoom { users: RefCell<Vec<Weak<User>>> }
impl ChatRoom {
    pub fn new() -> Rc<Self> { Rc::new(Self{users:RefCell::new(Vec::new())}) }
    fn broadcast(&self, from:&str, msg:&str) {
        for u in self.users.borrow_mut().iter() {
            if let Some(user)=u.upgrade() { user.receive(from,msg); }
        }
    }
}
pub struct User {
    name: String,
    room: Rc<ChatRoom>,
}
impl User {
    pub fn new(name:&str, room:Rc<ChatRoom>) -> Rc<Self> {
        let user = Rc::new(Self{name:name.into(), room});
        user.room.users.borrow_mut().push(Rc::downgrade(&user));
        user
    }
    pub fn send(&self, msg:&str) { self.room.broadcast(&self.name, msg); }
    fn receive(&self, from:&str, msg:&str) {
        println!("{} got from {}: {}", self.name, from, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn talk() {
        let room = ChatRoom::new();
        let a = User::new("Alice", room.clone());
        let _b = User::new("Bob", room.clone());
        a.send("Hi!");
    }
}
