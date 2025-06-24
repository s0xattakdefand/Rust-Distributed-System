// TODO: src
//! Skeleton algorithm in base impl; hooks overridable.

pub trait GameAI {
    fn collect_resources(&self) { println!("collect generic resources"); }
    fn build_structures(&self)  { println!("build generic structures"); }
    fn build_units(&self)       { println!("train generic units"); }

    fn turn(&self) {          // Template Method
        self.collect_resources();
        self.build_structures();
        self.build_units();
    }
}

pub struct Orcs; impl GameAI for Orcs {
    fn build_units(&self){ println!("train Axe Throwers"); }
}

pub struct Elves; impl GameAI for Elves {
    fn collect_resources(&self){ println!("gather mystical energy"); }
    fn build_units(&self){ println!("train Archers"); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn orc_turn() { Orcs.turn(); }
}
