// TODO: src
//! Double dispatch for area calculation.

pub trait Shape { fn accept(&self, v:&dyn Visitor) -> f64; }
pub trait Visitor { fn visit_circle(&self, c:&Circle)->f64; fn visit_square(&self, s:&Square)->f64; }

pub struct Circle(pub f64);  // radius
pub struct Square(pub f64);  // side

impl Shape for Circle { fn accept(&self,v:&dyn Visitor)->f64{v.visit_circle(self)} }
impl Shape for Square { fn accept(&self,v:&dyn Visitor)->f64{v.visit_square(self)} }

pub struct AreaVisitor;
impl Visitor for AreaVisitor {
    fn visit_circle(&self,c:&Circle)->f64{ std::f64::consts::PI*c.0*c.0 }
    fn visit_square(&self,s:&Square)->f64{ s.0*s.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn areas() {
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle(1.0)), Box::new(Square(2.0))];
        let a: f64 = shapes.iter().map(|s| s.accept(&AreaVisitor)).sum();
        assert!((a - (std::f64::consts::PI + 4.0)).abs() < 1e-6);
    }
}
