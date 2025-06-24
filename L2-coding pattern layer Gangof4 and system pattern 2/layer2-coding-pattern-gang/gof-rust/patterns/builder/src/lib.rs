//! Step-by-step Burger Builder.

#[derive(Default)]
pub struct Burger {
    bun: bool,
    patty: bool,
    veggies: bool,
    sauce: bool,
}
impl Burger {
    pub fn describe(&self) -> &'static str {
        "custom burger ready!"
    }
}

#[derive(Default)]
pub struct BurgerBuilder(Burger);

impl BurgerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bun(mut self) -> Self {
        self.0.bun = true;
        self
    }
    pub fn patty(mut self) -> Self {
        self.0.patty = true;
        self
    }
    pub fn veggies(mut self) -> Self {
        self.0.veggies = true;
        self
    }
    pub fn sauce(mut self) -> Self {
        self.0.sauce = true;
        self
    }

    pub fn build(self) -> Result<Burger, &'static str> {
        if self.0.bun && self.0.patty {
            Ok(self.0)
        } else {
            Err("bun + patty required")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn burger_ok() {
        let b = BurgerBuilder::default()
            .bun()
            .patty()
            .veggies()
            .build()
            .unwrap();
        assert_eq!(b.describe(), "custom burger ready!");
    }
}
