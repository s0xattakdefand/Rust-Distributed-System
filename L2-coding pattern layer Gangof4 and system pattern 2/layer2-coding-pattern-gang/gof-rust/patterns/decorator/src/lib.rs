// TODO: src
//! Coffee ↦ milk ↦ sugar decorators.

pub trait Beverage { fn cost(&self) -> f32; }

pub struct Espresso;
impl Beverage for Espresso { fn cost(&self) -> f32 { 1.50 } }

pub struct Milk<B: Beverage>(B);
impl<B: Beverage> Beverage for Milk<B> {
    fn cost(&self) -> f32 { self.0.cost() + 0.40 }
}
pub struct Sugar<B: Beverage>(B);
impl<B: Beverage> Beverage for Sugar<B> {
    fn cost(&self) -> f32 { self.0.cost() + 0.20 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn price() {
        let coffee = Sugar(Milk(Espresso));
        assert!((coffee.cost() - 2.10).abs() < 1e-6);
    }
}
