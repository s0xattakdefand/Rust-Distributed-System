//! Traffic-light State pattern.

pub trait LightState {
    fn next(&self) -> Box<dyn LightState>;
    fn color(&self) -> &'static str;
}

pub struct Red;
pub struct Green;
pub struct Yellow;

impl LightState for Red {
    fn next(&self) -> Box<dyn LightState> {
        Box::new(Green)
    }
    fn color(&self) -> &'static str {
        "RED"
    }
}
impl LightState for Green {
    fn next(&self) -> Box<dyn LightState> {
        Box::new(Yellow)
    }
    fn color(&self) -> &'static str {
        "GREEN"
    }
}
impl LightState for Yellow {
    fn next(&self) -> Box<dyn LightState> {
        Box::new(Red)
    }
    fn color(&self) -> &'static str {
        "YELLOW"
    }
}

pub struct TrafficLight {
    state: Box<dyn LightState>,
}
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: Box::new(Red),
        }
    }
    pub fn tick(&mut self) {
        self.state = self.state.next();
    }
    pub fn color(&self) -> &'static str {
        self.state.color()
    }
}
impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cycle() {
        let mut tl = TrafficLight::default();
        tl.tick();
        assert_eq!(tl.color(), "GREEN");
        tl.tick();
        assert_eq!(tl.color(), "YELLOW");
        tl.tick();
        assert_eq!(tl.color(), "RED");
    }
}
