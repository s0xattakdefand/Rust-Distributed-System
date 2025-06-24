// TODO: src
//! Cloning complicated objects – here, arbitrary Shapes.

pub trait Shape: ShapeClone {
    fn area(&self) -> f64;
}

pub trait ShapeClone { fn clone_box(&self) -> Box<dyn Shape>; }
impl<T> ShapeClone for T
where T: 'static + Shape + Clone {
    fn clone_box(&self) -> Box<dyn Shape> { Box::new(self.clone()) }
}
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self { self.clone_box() }
}

#[derive(Clone)]
pub struct Circle { pub r: f64 }
impl Shape for Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r } }

#[derive(Clone)]
pub struct Square { pub side: f64 }
impl Shape for Square { fn area(&self) -> f64 { self.side * self.side } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn clone_ok() {
        let c1: Box<dyn Shape> = Box::new(Circle { r: 2.0 });
        let c2 = c1.clone();
        assert!((c1.area() - c2.area()).abs() < 1e-6);
    }
}
