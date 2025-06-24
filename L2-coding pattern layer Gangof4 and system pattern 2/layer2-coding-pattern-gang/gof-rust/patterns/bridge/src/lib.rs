// TODO: src
//! RemoteControl (abstraction) decoupled from Device (implementation).

pub trait Device {
    fn is_on(&self) -> bool;
    fn power(&mut self);
}

pub struct Tv { on: bool }
impl Device for Tv {
    fn is_on(&self) -> bool { self.on }
    fn power(&mut self) { self.on = !self.on; }
}

pub struct Radio { on: bool }
impl Device for Radio {
    fn is_on(&self) -> bool { self.on }
    fn power(&mut self) { self.on = !self.on; }
}

/// Bridge
pub struct RemoteControl<D: Device> { dev: D }
impl<D: Device> RemoteControl<D> {
    pub fn new(dev: D) -> Self { Self { dev } }
    pub fn toggle_power(&mut self) { self.dev.power(); }
    pub fn status(&self) -> bool { self.dev.is_on() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn works() {
        let mut rc = RemoteControl::new(Tv{on:false});
        rc.toggle_power();
        assert!(rc.status());
    }
}
