//! Command pattern – lamp toggle + undo.

pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

#[derive(Default)]
pub struct Lamp {
    pub on: bool,
}
impl Lamp {
    pub fn new() -> Self {
        Self::default()
    }
    fn toggle(&mut self) {
        self.on = !self.on;
    }
}

pub struct ToggleCommand<'a> {
    lamp: &'a mut Lamp,
}
impl<'a> ToggleCommand<'a> {
    pub fn new(lamp: &'a mut Lamp) -> Self {
        Self { lamp }
    }
}
impl<'a> Command for ToggleCommand<'a> {
    fn execute(&mut self) {
        self.lamp.toggle();
    }
    fn undo(&mut self) {
        self.lamp.toggle();
    }
}

#[derive(Default)]
pub struct Remote<'a> {
    stack: Vec<Box<dyn Command + 'a>>,
}
impl<'a> Remote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn press(&mut self, mut cmd: Box<dyn Command + 'a>) {
        cmd.execute();
        self.stack.push(cmd);
    }
    pub fn undo(&mut self) {
        if let Some(mut c) = self.stack.pop() {
            c.undo();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn undo() {
        let mut lamp = Lamp::new();
        let mut remote = Remote::new();
        remote.press(Box::new(ToggleCommand::new(&mut lamp)));
        remote.undo();
        drop(remote);                 // ✅ release the mutable borrow
        assert!(!lamp.on);            // ✅ now we can access `lamp`

    }
}
