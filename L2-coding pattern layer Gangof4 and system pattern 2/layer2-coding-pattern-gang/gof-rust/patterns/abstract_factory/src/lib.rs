// TODO: src
//! Produces whole “families” of furniture without specifying concrete classes.

pub trait Chair {
    fn sit(&self) -> &'static str;
}
pub trait Sofa {
    fn lie(&self) -> &'static str;
}

struct VictorianChair;
impl Chair for VictorianChair {
    fn sit(&self) -> &'static str { "sit like a Victorian lady" }
}
struct ModernChair;
impl Chair for ModernChair {
    fn sit(&self) -> &'static str { "sit in minimalist style" }
}

struct VictorianSofa;
impl Sofa for VictorianSofa {
    fn lie(&self) -> &'static str { "lie on a velvet chaise longue" }
}
struct ModernSofa;
impl Sofa for ModernSofa {
    fn lie(&self) -> &'static str { "lie on low-profile couch" }
}

/// Abstract factory
pub trait FurnitureFactory {
    fn make_chair(&self) -> Box<dyn Chair>;
    fn make_sofa(&self)  -> Box<dyn Sofa>;
}

/// Concrete factories
pub struct VictorianFactory;
impl FurnitureFactory for VictorianFactory {
    fn make_chair(&self) -> Box<dyn Chair> { Box::new(VictorianChair) }
    fn make_sofa(&self)  -> Box<dyn Sofa>  { Box::new(VictorianSofa) }
}
pub struct ModernFactory;
impl FurnitureFactory for ModernFactory {
    fn make_chair(&self) -> Box<dyn Chair> { Box::new(ModernChair) }
    fn make_sofa(&self)  -> Box<dyn Sofa>  { Box::new(ModernSofa) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn families_match() {
        let m = ModernFactory;
        assert!(m.make_chair().sit().contains("minimalist"));
        let v = VictorianFactory;
        assert!(v.make_sofa().lie().contains("velvet"));
    }
}
